use std::fmt;

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

    fn real_add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text(self, text))
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
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

    pub fn status(&self) -> &str {
        self.state.as_ref().unwrap().status()
    }
}


trait State {
    fn add_text(self: Box<Self>, post: &mut Post, text: &str) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
    fn status(&self) -> &'static str;
}

// Manually implement Debug for trait State
impl fmt::Debug for dyn State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State: {}", self.status())
    }
}

// Manually implement Display for trait State
impl fmt::Display for dyn State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.status())
    }
}

struct Draft {}

impl State for Draft {
    fn add_text(self: Box<Self>, post: &mut Post, text: &str) -> Box<dyn State> {
        post.real_add_text(text);
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn status(&self) -> &'static str {
        "Draft"
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn add_text(self: Box<Self>, post: &mut Post, text: &str) -> Box<dyn State> {
        post.real_add_text(text);
        Box::new(Draft {})
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn status(&self) -> &'static str {
        "PendingReview"
    }
}

struct Published {}

impl State for Published {
    fn add_text(self: Box<Self>, _post: &mut Post, _text: &str) -> Box<dyn State> {
        eprintln!("Warning: cannot modify Post in Published state!");
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn status(&self) -> &'static str {
        "Published"
    }
}
