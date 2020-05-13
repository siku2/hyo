use std::{fmt, net::SocketAddr};

use futures::{sink::SinkExt, stream::StreamExt};

use async_std::net::TcpStream;
pub use async_tungstenite::{
    tungstenite::{
        self,
        handshake::server::{Callback, ErrorResponse, NoCallback, Request, Response},
        Message,
    },
    WebSocketStream,
};

pub struct Connection {
    pub addr: SocketAddr,
    pub stream: WebSocketStream<TcpStream>,
}

impl Connection {
    fn new(addr: SocketAddr, stream: WebSocketStream<TcpStream>) -> Self {
        Self { addr, stream }
    }

    pub async fn accept_hdr(
        addr: SocketAddr,
        raw: TcpStream,
        callback: impl Callback + Unpin,
    ) -> Result<Self, tungstenite::Error> {
        let stream = async_tungstenite::accept_hdr_async(raw, callback).await?;

        Ok(Self::new(addr, stream))
    }

    pub async fn accept(addr: SocketAddr, raw: TcpStream) -> Result<Self, tungstenite::Error> {
        Self::accept_hdr(addr, raw, NoCallback).await
    }

    pub async fn send_raw(&mut self, msg: Message) -> tungstenite::Result<()> {
        self.stream.send(msg).await
    }

    pub async fn receive_raw(&mut self) -> Option<tungstenite::Result<Message>> {
        self.stream.next().await
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("addr", &self.addr)
            .finish()
    }
}
