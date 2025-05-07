use tokio::sync::mpsc;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod connection;
mod socket;

mod sugar;
mod manage;

pub use connection::Connection;
pub type WSStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub trait GremlinStream: tokio::stream::Stream<Item=Option<crate::GremlinResult<crate::GValue>>> {}

impl<S> GremlinStream for S where S: tokio::stream::Stream<Item=Option<crate::GremlinResult<crate::GValue>>> {}


pub(self) use sugar::*;
pub(self) use socket::*;
