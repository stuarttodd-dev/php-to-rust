trait Message {
    fn render(&self) -> String;
}

struct Plain(String);

impl Message for Plain {
    fn render(&self) -> String {
        self.0.clone()
    }
}

struct Brackets {
    inner: Box<dyn Message>,
}

impl Message for Brackets {
    fn render(&self) -> String {
        format!("[{}]", self.inner.render())
    }
}

struct Exclaim {
    inner: Box<dyn Message>,
}

impl Message for Exclaim {
    fn render(&self) -> String {
        format!("{}!", self.inner.render())
    }
}

fn main() {
    let decorated: Box<dyn Message> = Box::new(Exclaim {
        inner: Box::new(Brackets {
            inner: Box::new(Plain(String::from("Patterns"))),
        }),
    });
    println!("{}", decorated.render());
}
