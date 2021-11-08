# rschat

A project for learning Rust and networking

## How to use

`cargo run` initializes the app at port `8080`.

There's also a docker image available (though not on the repository yet)


## Message protocol

This server uses a custom protocol.
It is a text-based protocol that works as follows:
 - Messages should be three lines long.
 - The first line contains the message type:
   - MSG, for text messages;
   - ERR, for errors;
   - LIN, for login;
   - LOU, for logout;
   - ACC, for accepting a login event or acknowledging a check;
   - CHK, for check, that is, asking if a connection is alive.
 - The second line identifies the message sender.
 - The third line contains the message contents.
 - If there is a line break on the message contents, write a literal `\n`.
 - Login is made by providing a password on the contents - **the password will be plain-text, be careful**.
 - The server sends an ACC upon login.
 - The server responds any CHK with an ACC.
 - MSGs will be broadcasted to all connected clients.
