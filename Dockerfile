FROM rust:1-slim-buster

WORKDIR /usr/src/rschat
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["rschat"]
