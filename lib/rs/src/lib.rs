#![allow(unstable)]
#![feature(slicing_syntax)]

pub use protocol::Protocol;
pub use transport::Transport;

pub mod protocol;
pub mod transport;

#[derive(Eq, PartialEq, Show)]
pub enum ThriftErr {
    TransportError(std::io::IoError),
    UnknownProtocol,
    InvalidData,
    NegativeSize,
    SizeLimit,
    BadVersion,
    NotImplemented,
    DepthLimit,
    InvalidUtf8(std::str::Utf8Error),
}

pub type TResult<T> = Result<T, ThriftErr>;
