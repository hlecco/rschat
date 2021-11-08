use std::cell::{Cell, RefCell};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;

use uuid::Uuid;

pub mod message;
pub use message::{Message, MsgType};

pub struct Client {
    stream: RefCell<TcpStream>,
    pub id: Uuid,
    pub name: String,
    counter: Cell<u16>,
    fail_counter: Cell<u8>,
    pub is_alive: Cell<bool>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream: RefCell::new(stream),
            id: Uuid::new_v4(),
            name: String::new(),
            counter: Cell::new(0),
            fail_counter: Cell::new(0),
            is_alive: Cell::new(true),
        }
    }

    pub fn get(&self) -> Option<Message> {
        let mut buffer = [0; 1024];
        match self.stream.borrow_mut().read(&mut buffer) {
            Ok(0) => None,
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buffer[..]);
                match msg.trim().is_empty() {
                    false => Message::from(&msg),
                    true => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn send(&self, msg: &Message) -> Result<usize> {
        let msg_string = msg.to_string();
        self.stream.borrow_mut().write(msg_string.as_bytes())
    }

    pub fn warn(&self) {
        if self.is_alive.get() {
            if self.counter.get() > 10 {
                let _ = self.send(&Message::new(MsgType::CHK, "server", ""));
                self.fail_counter.set(self.fail_counter.get() + 1);
                self.counter.set(0);
            } else {
                self.counter.set(self.counter.get() + 1);
                if self.fail_counter.get() > 5 {
                    self.is_alive.set(false);
                    println!("Client {} has been disconnected", self.id);
                    let _ = self.send(&Message::new(
                        MsgType::ERR,
                        "server",
                        "Disconnected from server due to unresponsiveness",
                    ));
                }
            }
        }
    }

    pub fn ok(&self) {
        self.fail_counter.set(0);
    }

    pub fn broadcast<'a>(msg: &Message, clients: &'a Vec<Client>) -> Vec<&'a Client> {
        let mut healthy: Vec<&Client> = Vec::new();

        for client in clients {
            if let Ok(_) = client.send(msg) {
                healthy.push(client);
            }
        }
        healthy
    }
}

pub struct User {
    login: String,
    password: String,
}

impl User {
    pub fn new(login: &str, password: &str) -> User {
        User {
            login: String::from(login),
            password: String::from(password),
        }
    }
}

pub struct Users {
    users: Vec<User>,
}

impl Users {
    pub fn new() -> Users {
        Users { users: Vec::new() }
    }

    pub fn add_user(&mut self, login: &str, password: &str) -> Result<()> {
        match self.user_exists(login) {
            false => Ok({
                self.users.push(User::new(login, password));
            }),
            true => Err(Error::from(ErrorKind::Other)),
        }
    }

    pub fn user_exists(&mut self, login: &str) -> bool {
        self.users.iter().any(|user| user.login == login)
    }

    pub fn check_credentials(&mut self, login: &str, password: &str) -> bool {
        self.users
            .iter()
            .any(|user| (user.login == login) && (user.password == password))
    }
}
