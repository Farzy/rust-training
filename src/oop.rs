mod blog;

use blog::Post;

pub fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    assert_eq!("Draft", post.status());

    post.request_review();
    assert_eq!("", post.content());
    assert_eq!("PendingReview", post.status());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!("Published", post.status());
}
