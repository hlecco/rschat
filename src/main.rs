use std::net::TcpListener;
use std::{thread, time};

use rschat::Client;
use rschat::Users;
use rschat::{Message, MsgType};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Can't bind address");
    listener.set_nonblocking(true).unwrap();

    let mut clients: Vec<Client> = Vec::new();
    let mut users = Users::new();

    for s in listener.incoming() {
        if let Ok(stream) = s {
            stream.set_nonblocking(true).unwrap();

            let new_client = Client::new(stream);
            if let Some(msg) = new_client.get() {
                let mut accepted = false;
                match msg.msg {
                    MsgType::LIN => {
                        if users.user_exists(&msg.from) {
                            if users.check_credentials(&msg.from, &msg.content) {
                                accepted = true;
                            }
                        } else {
                            users.add_user(&msg.from, &msg.content).unwrap();
                            accepted = true;
                        }
                    }
                    _ => {}
                }

                if accepted {
                    Client::broadcast(
                        &Message::new(MsgType::MSG, "server", &format!("Say hi to \"{}\"", &msg.from)),
                        &clients,
                    );
                    println!("New client {}", &new_client.id);
                    new_client
                        .send(&Message::new(MsgType::ACC, "", ""))
                        .unwrap();
                    clients.push(new_client);
                }
            }
        }

        for client in &clients {
            if let Some(msg) = client.get() {
                match msg.msg {
                    MsgType::MSG => {
                        println!("{}", msg.to_string());
                        Client::broadcast(&msg, &clients);
                    }
                    MsgType::CHK => {
                        client
                            .send(&Message::new(MsgType::ACC, "server", ""))
                            .unwrap();
                    }
                    MsgType::ACC => {
                        client.ok();
                    }
                    MsgType::LOU => {
                        client
                            .send(&Message::new(MsgType::LOU, "server", ""))
                            .unwrap();
                        client.is_alive.set(false);
                    }
                    _ => {}
                }
            }
            client.warn();
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}
