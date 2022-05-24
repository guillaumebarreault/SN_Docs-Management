FROM rust:1.59.0
COPY .. /home/kali/Documents/SN_Docs-Management/doc_management/src/
RUN rustup default 1.59.0
RUN cargo build –release 
EXPOSE 4100
CMD [‘’./target/release/monapp ‘’]
