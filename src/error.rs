use thiserror::Error;

use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum AsError {
    #[error("IoError(source: _source, backtrace: _backtrace)")]
    Io {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("fail to proxy ")]
    ProxyFail {},
}
