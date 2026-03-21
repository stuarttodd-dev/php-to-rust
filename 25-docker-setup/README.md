# 25-docker-setup

![php-to-rust logo](../images/php-to-rust.png)

Containerizing a Rust application with Docker.

This exercise demonstrates how to package a Rust application into a Docker container.

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