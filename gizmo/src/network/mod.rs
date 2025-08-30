use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use uuid::Uuid;

mod cmd;
mod receiver;
mod sender;
mod socket;

mod manage;

pub(crate) use cmd::Cmd;
use gizmio::Response;
pub(self) use receiver::ReceiverLoop;
pub(self) use sender::SenderLoop;
pub use socket::*;

use crate::GremlinResult;

pub type WSStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub type RequestMap = Arc<DashMap<Uuid, Sender<GremlinResult<Response>>>>;
