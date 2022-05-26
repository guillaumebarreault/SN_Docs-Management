FROM rust:1.59.0 as builder

RUN mkdir /usr/src/doc-service
WORKDIR /usr/src/doc-service
COPY . .

RUN rustup default 1.59.0
RUN cargo build --release

EXPOSE 4100

WORKDIR /usr/src/doc-service

CMD ["./target/release/apiminio"]