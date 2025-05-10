use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod connection;
mod socket;

mod manage;
mod sugar;

pub use connection::Connection;
pub type WSStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub trait GremlinStream:
    tokio::stream::Stream<Item = Option<crate::GremlinResult<crate::GValue>>>
{
}

impl<S> GremlinStream for S where
    S: tokio::stream::Stream<Item = Option<crate::GremlinResult<crate::GValue>>>
{
}

pub(self) use socket::*;
pub(self) use sugar::*;
