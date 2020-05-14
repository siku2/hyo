use futures::{channel::mpsc, StreamExt};

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CloseEvent, ErrorEvent, Event, MessageEvent, WebSocket};

#[derive(Clone, Copy, Debug)]
pub enum ReadyState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Clone, Debug)]
pub enum RawWSEvent {
    Close(CloseEvent),
    Error(ErrorEvent),
    Message(MessageEvent),
    Open(Event),
}

#[derive(Debug)]
pub struct RawWS {
    pub ws: WebSocket,
    pub events: mpsc::UnboundedReceiver<RawWSEvent>,
    _onclose: Closure<dyn FnMut(CloseEvent)>,
    _onerror: Closure<dyn FnMut(ErrorEvent)>,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
    _onopen: Closure<dyn FnMut(Event)>,
}

impl RawWS {
    pub fn from_ws(ws: WebSocket) -> Self {
        let (tx_, rx) = mpsc::unbounded::<RawWSEvent>();

        let tx = tx_.clone();
        let onclose_callback: Closure<dyn FnMut(CloseEvent)> = Closure::new(move |e| {
            tx.unbounded_send(RawWSEvent::Close(e)).unwrap();
        });

        let tx = tx_.clone();
        let onerror_callback: Closure<dyn FnMut(ErrorEvent)> = Closure::new(move |e| {
            tx.unbounded_send(RawWSEvent::Error(e)).unwrap();
        });

        let tx = tx_.clone();
        let onmessage_callback: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |e| {
            tx.unbounded_send(RawWSEvent::Message(e)).unwrap();
        });

        let tx = tx_;
        let onopen_callback: Closure<dyn FnMut(Event)> = Closure::new(move |e| {
            tx.unbounded_send(RawWSEvent::Open(e)).unwrap();
        });

        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

        Self {
            ws,
            events: rx,
            _onclose: onclose_callback,
            _onerror: onerror_callback,
            _onmessage: onmessage_callback,
            _onopen: onopen_callback,
        }
    }

    pub fn new(url: &str) -> Result<Self, WSError> {
        WebSocket::new(url)
            .map_err(|_| WSError::Unknown)
            .map(Self::from_ws)
    }

    pub async fn connect(url: &str) -> Result<Self, WSError> {
        let mut ws = Self::new(url)?;
        ws.wait_open().await?;
        Ok(ws)
    }

    async fn wait_open(&mut self) -> Result<Event, WSError> {
        let event = self.events.next().await.ok_or(WSError::Unknown)?;
        match event {
            RawWSEvent::Open(e) => Ok(e),
            _ => Err(WSError::UnexpectedEvent(event)),
        }
    }

    async fn wait_close(&mut self) -> Result<CloseEvent, WSError> {
        let event = self.events.next().await.ok_or(WSError::Unknown)?;
        match event {
            RawWSEvent::Close(e) => Ok(e),
            _ => Err(WSError::UnexpectedEvent(event)),
        }
    }

    pub async fn close(&mut self) -> Result<CloseEvent, WSError> {
        self.ws.close().map_err(|_| WSError::Unknown)?;
        self.wait_close().await
    }

    fn ready_state(&self) -> ReadyState {
        use ReadyState::*;

        match self.ws.ready_state() {
            0 => Connecting,
            1 => Open,
            2 => Closing,
            3 => Closed,
            _ => unreachable!(),
        }
    }

    fn check_open(&self) -> Result<(), WSError> {
        match self.ready_state() {
            ReadyState::Open => Ok(()),
            _ => Err(WSError::NotConnected),
        }
    }

    pub fn send_text(&self, data: &str) -> Result<(), WSError> {
        self.check_open()?;
        self.ws.send_with_str(data).map_err(|_| WSError::Unknown)
    }
    pub fn send_binary(&self, data: &[u8]) -> Result<(), WSError> {
        self.check_open()?;
        self.ws
            .send_with_u8_array(data)
            .map_err(|_| WSError::Unknown)
    }

    pub fn send_message(&self, msg: &Message) -> Result<(), WSError> {
        match msg {
            Message::Text(data) => self.send_text(data),
            Message::Binary(data) => self.send_binary(data),
        }
    }
}

impl Drop for RawWS {
    fn drop(&mut self) {
        let ws = &mut self.ws;
        ws.set_onclose(None);
        ws.set_onerror(None);
        ws.set_onmessage(None);
        ws.set_onopen(None);

        if let Err(e) = ws.close() {
            log::error!("failed to close websocket in drop: {:?}", e);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WSError {
    #[error("not connected")]
    NotConnected,

    #[error("received unexpected event: {0:?}")]
    UnexpectedEvent(RawWSEvent),
    #[error("unknown error")]
    Unknown,
}

pub enum Message {
    Binary(Vec<u8>),
    Text(String),
}
