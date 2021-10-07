# rschat

A project for learning Rust and networking

## How to use

In this version, the app only gets every packet from the clients and
broadcasts them.

`cargo run` initializes the app at `127.0.0.1:8080`.

Use multiple instances of `telnet localhost 8080` and see what happens!


### Note

There's also a docker image available (though not on the repository yet)
