FROM rust:1.59.0 as builder

RUN mkdir /usr/src/doc-service
WORKDIR /usr/src/doc-service
COPY . .

RUN rustup default 1.58.1
RUN cargo build --release

EXPOSE 5100


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /usr/src/doc-service/target/release/doc_management /usr/src/doc-service/
WORKDIR /usr/src/doc-service

EXPOSE 5100

CMD ["./doc_management"]