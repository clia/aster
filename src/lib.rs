#![feature(backtrace)]

pub mod config;
pub mod error;

#[tokio::main]
pub async fn proxy_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
