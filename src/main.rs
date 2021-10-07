use std::net::TcpListener;

use rschat::Client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Can't bind address");
    listener.set_nonblocking(true).unwrap();

    let mut clients: Vec<Client> = Vec::new();

    for s in listener.incoming() {
        if let Ok(stream) = s {
            stream.set_nonblocking(true).unwrap();

            let new_client = Client::new(stream);
            new_client.send("Welcome!\n").unwrap();
            new_client.broadcast("A friend has arrived!\n", &clients);

            clients.push(new_client);
        }

        for client in &clients {
            if let Some(msg) = client.get() {
                client.broadcast(&msg, &clients);
            }
        }
    }
}
