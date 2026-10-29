FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src ./src
RUN touch src/main.rs && cargo build --release
# Binary filename always matches [package].name — keep Dockerfile in sync automatically.
RUN PKG=$(grep -m1 '^name = ' Cargo.toml | cut -d'"' -f2) && \
    test -f "target/release/$PKG" && \
    cp "target/release/$PKG" /app/server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/server /app/server
EXPOSE 8080
CMD ["./server"]
