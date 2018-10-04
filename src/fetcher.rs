use tokio::prelude::{Async, AsyncSink, Stream};
use tokio::timer::Interval;

use self::super::Cluster;
use cmd::{new_cluster_nodes_cmd, Cmd};
use com::*;
use resp::RESP_BULK;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Copy)]
enum FetchState {
    Ready,
    Wait,
}

pub struct Fetcher {
    cluster: Rc<Cluster>,
    cursor: usize,
    servers: Vec<String>,
    state: FetchState,
    info_cmd: Cmd,
    internal: Interval,
}

impl Fetcher {
    pub fn new(cluster: Rc<Cluster>) -> Fetcher {
        let servers = cluster.cc.servers.clone();
        let duration = Duration::from_secs(cluster.cc.fetch);
        Fetcher {
            cluster: cluster,
            cursor: 0,
            servers: servers,
            state: FetchState::Ready,
            info_cmd: new_cluster_nodes_cmd(),
            internal: Interval::new(Instant::now() + duration, duration),
        }
    }
}

impl Stream for Fetcher {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if let None = try_ready!(self.internal.poll().map_err(|err| {
            error!("fetch by internal fail due {:?}", err);
            Error::Critical
        })) {
            return Ok(Async::Ready(None));
        }

        // info!("trying to fetch");
        loop {
            // debug!("fetch status cursor={} cmd={:?}", self.cursor, self.info_cmd);
            match self.state {
                FetchState::Ready => {
                    if self.cursor == 0 {
                        self.info_cmd = new_cluster_nodes_cmd();
                    }

                    let cursor = self.cursor;
                    if cursor == self.servers.len() {
                        debug!("fail to update slots map but pass the turn");
                        self.cursor = 0;
                        self.info_cmd = new_cluster_nodes_cmd();
                        return Ok(Async::Ready(Some(())));
                    }
                    let addr = self.servers.get(cursor).cloned().unwrap();
                    debug!("trying to execute cmd to {}", addr);
                    match self.cluster.execute(&addr, self.info_cmd.clone())? {
                        AsyncSink::NotReady(_) => return Ok(Async::NotReady),
                        AsyncSink::Ready => {
                            self.state = FetchState::Wait;
                        }
                    }
                    self.cursor += 1;
                }
                FetchState::Wait => {
                    let cmd = self.info_cmd.clone();
                    if !cmd.is_done() {
                        return Ok(Async::NotReady);
                    }

                    let resp = cmd.swap_reply().expect("fetch result never be empty for an done cmd");

                    if resp.rtype != RESP_BULK {
                        warn!("fetch fail due to bad resp {:?}", resp);
                        self.state = FetchState::Ready;
                        continue;
                    }

                    let mut slots_map = self.cluster.slots.borrow_mut();
                    let updated = slots_map.try_update_all(resp.data.as_ref().expect("never be empty"));
                    if updated {
                        info!("success update slotsmap due slots map is changed");
                        self.cursor = 0;
                        return Ok(Async::Ready(Some(())));
                    } else {
                        debug!("skip to update due cluster slots map is never changed");
                        self.state = FetchState::Ready;
                    }
                }
            }
        }
    }
}
