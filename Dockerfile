FROM rust:1.81-slim

WORKDIR /app

# install sqlite dependency
RUN apt-get update && apt-get install -y libsqlite3-dev && rm -rf /var/lib/apt/lists/*

# copy workspace files
COPY Cargo.toml Cargo.toml
COPY backend ./backend
COPY shared ./shared

# create a dummy frontend so workspace resolves without building it
COPY frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir -p frontend/src && echo 'fn main(){}' > frontend/src/main.rs

# build only the backend binary
RUN cargo build --release --bin forgeurl

EXPOSE 7878

CMD ["./target/release/forgeurl"]