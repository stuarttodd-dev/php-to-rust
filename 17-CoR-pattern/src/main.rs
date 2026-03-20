trait Handler {
    fn handle(&self, req: &str) -> Option<String>;
}

struct LoggingLayer {
    next: Box<dyn Handler>,
}

impl Handler for LoggingLayer {
    fn handle(&self, req: &str) -> Option<String> {
        println!("[log] incoming: {}", req);
        self.next.handle(req)
    }
}

struct AuthLayer {
    next: Box<dyn Handler>,
}

impl Handler for AuthLayer {
    fn handle(&self, req: &str) -> Option<String> {
        if req.contains("TOKEN") {
            self.next.handle(req)
        } else {
            Some(String::from("rejected: missing token"))
        }
    }
}

struct Business;
impl Handler for Business {
    fn handle(&self, req: &str) -> Option<String> {
        Some(format!("handled: {}", req))
    }
}

fn main() {
    let chain: Box<dyn Handler> = Box::new(LoggingLayer {
        next: Box::new(AuthLayer {
            next: Box::new(Business),
        }),
    });
    println!("{:?}", chain.handle("TOKEN order-42"));
    println!("{:?}", chain.handle("no auth"));
}
