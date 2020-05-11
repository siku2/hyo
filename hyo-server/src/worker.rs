use std::{
    collections::HashMap,
    fmt,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future,
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt, TryStreamExt},
};

use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use async_tungstenite::{
    tungstenite::{
        self,
        handshake::server::{Callback, NoCallback, Request, Response},
        http::HeaderMap,
        Message,
    },
    WebSocketStream,
};

struct Connection<S> {
    pub addr: SocketAddr,
    pub headers: HeaderMap,
    pub stream: WebSocketStream<S>,
}

impl Connection<TcpStream> {
    fn new(addr: SocketAddr, headers: HeaderMap, stream: WebSocketStream<TcpStream>) -> Self {
        Self {
            addr,
            headers,
            stream,
        }
    }

    pub async fn accept_hdr(
        addr: SocketAddr,
        raw: TcpStream,
        callback: impl Callback + Unpin,
    ) -> Result<Self, tungstenite::Error> {
        let mut headers = None;
        let stream =
            async_tungstenite::accept_hdr_async(raw, |req: &Request, mut resp: Response| {
                resp = callback.on_request(req, resp)?;

                headers = Some(req.headers().clone());
                Ok(resp)
            })
            .await?;

        Ok(Self::new(addr, headers.unwrap(), stream))
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

impl<S> fmt::Debug for Connection<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("addr", &self.addr)
            .field("headers", &self.headers)
            .finish()
    }
}

async fn test(addr: SocketAddr, stream: TcpStream) {
    let conn = Connection::accept(addr, stream).await;
    log::info!("Connection: {:?}", conn);
}

async fn run_loop(addr: impl ToSocketAddrs) -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind(addr).await?;

    match listener.local_addr() {
        Ok(addr) => log::info!("websocket listening at {}", addr),
        Err(_) => log::info!("websocket listening at unknown address"),
    }

    while let Ok((stream, addr)) = listener.accept().await {
        task::spawn(test(addr, stream));
    }

    Ok(())
}

fn run() -> Result<(), anyhow::Error> {
    task::block_on(async { run_loop("localhost:8800").await })
}

pub fn run_in_thread() {
    use std::thread;

    thread::spawn(|| match run() {
        Ok(_) => (),
        Err(e) => log::error!("worker failed: {:?}", e),
    });
}
