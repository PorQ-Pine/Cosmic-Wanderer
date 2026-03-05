use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

use niri_ipc::socket::Socket;
use niri_ipc::{Request, Response};

pub struct NiriWatcher {
    send: Sender<()>,
}

impl NiriWatcher {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel();
        (Self { send: tx }, rx)
    }

    pub fn start(&mut self) {
        let mut socket = Socket::connect().unwrap();
        let reply = socket.send(Request::EventStream).unwrap();
        if matches!(reply, Ok(Response::Handled)) {
            let mut read_event = socket.read_events();
            while let Ok(event) = read_event() {
                match event {
                    niri_ipc::Event::WindowFocusChanged { id } => {
                        if id.is_some() {
                            self.send.send(()).unwrap();
                            thread::sleep(Duration::from_millis(300));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
