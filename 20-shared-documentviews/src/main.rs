use std::rc::Rc;

#[derive(Debug)]
struct Document {
    content: String,
}

struct TextViewer {
    document: Rc<Document>,
}

struct WordCounter {
    document: Rc<Document>,
}

impl TextViewer {
    fn preview(&self) {
        println!("preview: {}", self.document.content);
    }
}

impl WordCounter {
    fn count_words(&self) -> usize {
        self.document.content.split_whitespace().count()
    }
}

fn main() {
    let doc = Rc::new(Document {
        content: "Rust smart pointers help model ownership clearly".to_string(),
    });

    let viewer = TextViewer {
        document: Rc::clone(&doc),
    };
    let counter = WordCounter {
        document: Rc::clone(&doc),
    };

    println!("strong_count before use={}", Rc::strong_count(&doc));
    viewer.preview();
    println!("word_count={}", counter.count_words());

    drop(counter);
    println!("strong_count after drop counter={}", Rc::strong_count(&doc));
}
