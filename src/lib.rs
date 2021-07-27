#![feature(backtrace)]


pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/aster.cache.rs"));
}

pub mod error;

#[tokio::main]
pub async fn proxy_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
