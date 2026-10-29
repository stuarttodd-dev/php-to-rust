trait Playable {
    fn play(&self);
}

struct AudioBook {
    title: String,
    author: String,
}

struct VideoGame {
    name: String,
    platform: String,
}

impl Playable for AudioBook {
    fn play(&self) {
        println!("Now playing book: {} (by {})...", self.title, self.author);
    }
}

impl Playable for VideoGame {
    fn play(&self) {
        println!("Launching game: {} on {}...", self.name, self.platform);
    }
}

fn consume_media<T: Playable>(item: T) {
    item.play();
}

struct Metadata<'a> {
    description: &'a str,
}

fn main() {
    let book = AudioBook {
        title: "The Rust Book".into(),
        author: "Rust Community".into(),
    };
    let game = VideoGame {
        name: "Rusty Quest".into(),
        platform: "PC".into(),
    };
    consume_media(book);
    consume_media(game);

    let desc: &'static str = "A great course";
    let meta = Metadata { description: desc };
    println!("{}", meta.description);
}
