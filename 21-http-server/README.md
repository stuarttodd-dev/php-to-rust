# 21 - HTTP Server

![php-to-rust logo](../images/php-to-rust.png)

This example implements a basic HTTP server in Rust using TCP sockets and threading, handling multiple concurrent client connections.

Features:
- TCP listener on `127.0.0.1:8080`
- Handles GET requests to `/`, `/index.html`, and `/about.html`
- Returns 404 for unknown routes
- Multi-threaded request handling

## Run

From this folder:

```bash
cargo run
```

Server starts on `http://127.0.0.1:8080`.

## Test with curl

In another terminal:

```bash
# Test home page
curl http://127.0.0.1:8080/

# Test index
curl http://127.0.0.1:8080/index.html

# Test about page
curl http://127.0.0.1:8080/about.html

# Test 404
curl http://127.0.0.1:8080/nonexistent
```

Expected output shows HTML responses for valid routes and 404 for invalid ones.
