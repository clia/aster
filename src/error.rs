use thiserror::Error;

use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};

#[derive(Error, Debug)]
pub enum AsError {
    Io {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("fail to proxy ")]
    ProxyFail {
    }
}

impl Display for AsError {
    fn fmt(&self, writer: &mut Formatter<'_>) -> std::fmt::Result {
        write!(writer, "{:?}", self)
    }
}
