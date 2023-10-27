pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PengingReviewPost {
        PengingReviewPost {
            content: self.content,
        }
    }
}

pub struct PengingReviewPost {
    content: String,
}

impl PengingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}