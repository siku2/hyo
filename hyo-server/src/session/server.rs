use super::{
    connection::{Connection, ErrorResponse, Request, Response},
    Player,
    Session,
};
use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use std::{collections::HashMap, net::SocketAddr};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct SessionServer {
    sessions: HashMap<Uuid, Session>,
}

impl SessionServer {
    pub fn iter_public_sessions(&self) -> impl Iterator<Item = &Session> {
        self.sessions.values().filter(|sess| sess.settings.public)
    }

    fn new_session_id(&self) -> Uuid {
        loop {
            let id = Uuid::new_v4();
            if !self.sessions.contains_key(&id) {
                return id;
            }
        }
    }
}

async fn handle_connection(addr: SocketAddr, stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut session = None;
    let mut player_id = None;

    let conn = Connection::accept_hdr(
        addr,
        stream,
        |req: &Request, resp: Response| -> Result<Response, ErrorResponse> {
            // TODO use response builder for custom error code and such
            // session = self
            //     .sessions
            //     .get(&Uuid::new_v4())
            //     .map(Some)
            //     .ok_or_else(|| ErrorResponse::new(Some(String::from("invalid session id"))))?;
            player_id = Some(Uuid::new_v4());
            Ok(resp)
        },
    )
    .await?;

    let session: &Session = session.unwrap();

    let player = Player {
        id: player_id.unwrap(),
        conn,
    };

    Ok(())
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind(addr).await?;

    match listener.local_addr() {
        Ok(addr) => log::info!("websocket listening at {}", addr),
        Err(_) => log::info!("websocket listening at unknown address"),
    }

    while let Ok((stream, addr)) = listener.accept().await {
        task::spawn(handle_connection(addr, stream));
    }

    Ok(())
}

pub fn run_in_thread() {
    use std::thread;

    thread::spawn(|| {
        let result = task::block_on(async { accept_loop("localhost:8800").await });
        match result {
            Ok(_) => (),
            Err(e) => log::error!("worker failed: {:?}", e),
        }
    });
}
