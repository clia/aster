use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AsterConfig {
    pub workdir: String,
    pub logdir: String,
    pub clusters: Vec<ClusterConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CacheType {
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "memcache")]
    Memcache,
    #[serde(rename = "memcache_binary")]
    MemcacheBinary,
    #[serde(rename = "redis_cluster")]
    RedisCluster,
}

impl Default for CacheType {
    fn default() -> CacheType {
        CacheType::RedisCluster
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClusterConfig {
    pub name: String,
    pub listen_addr: String,
    pub hash_tag: Option<String>,

    pub thread: Option<usize>,
    pub cache_type: CacheType,

    pub read_timeout: Option<u64>,
    pub write_timeout: Option<u64>,

    #[serde(default)]
    pub servers: Vec<String>,

    // cluster special
    pub fetch_interval: Option<u64>,
    pub read_from_slave: Option<bool>,

    // proxy special
    pub ping_fail_limit: Option<u8>,
    pub ping_interval: Option<u64>,
    pub ping_succ_interval: Option<u64>,

    // command not support now
    pub dial_timeout: Option<u64>,
    // dead option: not support other proto
    pub listen_proto: Option<String>,

    // dead option: always 1
    pub node_connections: Option<usize>,
}
