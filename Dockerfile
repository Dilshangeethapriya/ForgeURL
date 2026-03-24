FROM rust:1.82-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libsqlite3-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.toml
COPY backend ./backend
COPY shared ./shared

COPY frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir -p frontend/src && echo 'fn main(){}' > frontend/src/main.rs

RUN cargo build --release --bin forgeurl

RUN mkdir -p /app/data

EXPOSE 7878

CMD ["./target/release/forgeurl"]

