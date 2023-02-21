FROM rust:1.67.0
WORKDIR /usr/src/spot-finder
COPY . .
RUN cargo run
