use std::io::prelude::*;
use std::net::TcpStream;
use std::cell::RefCell;
use std::io::Result;

use uuid::Uuid;

pub struct Client {
    stream: RefCell<TcpStream>,
    id: Uuid,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream: RefCell::new(stream),
            id: Uuid::new_v4(),
        }
    }

    pub fn get(&self) -> Option<String> {
        let mut buffer = [0; 1024];
        match self.stream.borrow_mut().read(&mut buffer) {
            Ok(0) => None,
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buffer[..]);
                match msg.trim().is_empty() {
                    false => {
                        Some(msg.to_string())
                    },
                    true => None
                }
            },
            Err(_) => None,
        }
    }

    pub fn send(&self, msg: &str) -> Result<usize> {
        self.stream.borrow_mut().write(msg.as_bytes())
    }

    pub fn broadcast<'a>(&self, msg: &str, clients: &'a Vec<Client>) -> Vec<&'a Client> {
        let mut healthy: Vec<&Client> = Vec::new();

        for client in clients {
            if client.id != self.id {
                if let Ok(_) = client.send(msg) {
                    healthy.push(client);
                }
            }
        }
        healthy
    }
}
