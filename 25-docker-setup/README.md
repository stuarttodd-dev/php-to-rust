# 25-docker-setup

![php-to-rust logo](../images/php-to-rust.png)

Containerizing a Rust application with Docker.

This exercise demonstrates how to package a Rust application into a Docker container with optimized caching layers for faster rebuilds.

## Running

To build the Docker image:

```bash
docker build -t docker-setup .
```

To run the container:

```bash
docker run --rm docker-setup
```

The application will print "Hello, world!" to the console.

## Optimization

The Dockerfile uses multi-stage caching:
- Copies `Cargo.toml` and `Cargo.lock` first to cache dependencies
- Creates a dummy `main.rs` to allow initial dependency build
- Copies source code and rebuilds with the real code
- Uses `.dockerignore` to exclude unnecessary files from build context