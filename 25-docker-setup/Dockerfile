FROM rust:1.78-bookworm
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src ./src
RUN touch src/main.rs && cargo build --release
EXPOSE 8080
CMD ["./target/release/docker-setup"]