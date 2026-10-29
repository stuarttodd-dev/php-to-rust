FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shop ./shop
COPY discounts ./discounts
COPY shop-api ./shop-api
ENV SQLX_OFFLINE=true
RUN cargo build -p shop-api --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home shop
COPY --from=builder /app/target/release/shop-api /usr/local/bin/shop-api
USER shop
CMD ["shop-api"]
