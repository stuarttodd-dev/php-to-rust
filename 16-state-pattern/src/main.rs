struct DraftPost {
    content: String,
}
struct PendingReviewPost {
    content: String,
}
struct PublishedPost {
    content: String,
}

enum PostState {
    Draft(DraftPost),
    PendingReview(PendingReviewPost),
    Published(PublishedPost),
}

pub struct Post {
    state: PostState,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: PostState::Draft(DraftPost {
                content: String::new(),
            }),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if let PostState::Draft(ref mut draft) = self.state {
            draft.content.push_str(text);
        }
    }
    pub fn request_review(&mut self) {
        let ph = PostState::Draft(DraftPost {
            content: String::new(),
        });
        if let PostState::Draft(draft) = std::mem::replace(&mut self.state, ph) {
            self.state = PostState::PendingReview(PendingReviewPost {
                content: draft.content,
            });
        }
    }
    pub fn approve(&mut self) {
        let ph = PostState::Draft(DraftPost {
            content: String::new(),
        });
        if let PostState::PendingReview(p) = std::mem::replace(&mut self.state, ph) {
            self.state = PostState::Published(PublishedPost {
                content: p.content,
            });
        }
    }
    pub fn content(&self) -> &str {
        match &self.state {
            PostState::Draft(s) => &s.content,
            PostState::PendingReview(s) => &s.content,
            PostState::Published(s) => &s.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("Hello, state machine!");
    println!("Draft: {}", post.content());
    post.request_review();
    post.add_text(" Ignored.");
    post.approve();
    println!("Published: {}", post.content());
}
