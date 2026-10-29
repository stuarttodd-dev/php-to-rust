use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    // Consume headers until blank line.
    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header == "\r\n" || header.is_empty() {
            break;
        }
    }

    let (status, body) = if request_line.starts_with("GET / HTTP/1.1")
        || request_line.starts_with("GET /index.html HTTP/1.1")
    {
        ("HTTP/1.1 200 OK", "<h1>Index</h1>")
    } else if request_line.starts_with("GET /about.html HTTP/1.1") {
        ("HTTP/1.1 200 OK", "<h1>About</h1>")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "<h1>Not Found</h1>")
    };

    let response = format!(
        "{status}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("HTTP server on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("client error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("accept failed: {}", e),
        }
    }

    Ok(())
}
