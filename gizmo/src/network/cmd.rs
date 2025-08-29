use bytes::Bytes;
use gizmio::{Request, Response};
use tokio::sync::mpsc::Sender;

use crate::GremlinResult;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Cmd {
    Msg((Sender<GremlinResult<Response>>, Request)),
    Ping(Bytes),
    Shutdown,
}
