pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().is_editable() {
            self.content.push_str(text);
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn is_editable(&self) -> bool;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_editable(&self) -> bool {
        true
    }
}

struct PendingReview {
    no_of_approvals: u8,
}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview { no_of_approvals: 0 }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.no_of_approvals < 2 {
            Box::new(PendingReview {
                no_of_approvals: self.no_of_approvals + 1,
            })
        } else {
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn is_editable(&self) -> bool {
        false
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_editable(&self) -> bool {
        false
    }
}
