use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsterConfig {
    pub workdir: String,
    pub logdir: String,
    pub clusters: Vec<ClusterConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub name: String,
    pub listen_addr: String,
    pub servers: Vec<String>,
    pub node_connections: i64,
}
