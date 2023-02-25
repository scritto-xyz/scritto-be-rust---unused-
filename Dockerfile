FROM rust:buster

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/scritto"]