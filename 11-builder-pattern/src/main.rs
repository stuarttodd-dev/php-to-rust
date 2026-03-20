#[derive(Debug)]
pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
}

pub struct WindowConfigBuilder {
    title: String,
    width: Option<u32>,
    height: Option<u32>,
}

impl WindowConfigBuilder {
    pub fn new(title: String) -> Self {
        WindowConfigBuilder { title, width: None, height: None }
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
    pub fn build(self) -> WindowConfig {
        WindowConfig {
            title: self.title,
            width: self.width.unwrap_or(800),
            height: self.height.unwrap_or(600),
        }
    }
}

fn main() {
    let cfg = WindowConfigBuilder::new(String::from("Game"))
        .width(1024)
        .height(768)
        .build();
    // Read fields so the compiler does not warn (Debug alone does not count as a "read" for dead_code).
    println!("Window '{}' is {}×{}", cfg.title, cfg.width, cfg.height);
    println!("{:?}", cfg);
}
