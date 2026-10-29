FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN rustup component add rustfmt clippy \
 && rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall -y trunk wasm-pack sqlx-cli

WORKDIR /workspace
