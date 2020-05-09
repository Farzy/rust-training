mod blog;

use blog::Post;

pub fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.\n");
    assert_eq!("Draft", post.status());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("PendingReview", post.status());
    assert_eq!("", post.content());

    post.add_text("Some more salad…\n");
    assert_eq!("Draft", post.status());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("PendingReview", post.status());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Published", post.status());
    assert_eq!("I ate a salad for lunch today.\nSome more salad…\n", post.content());

    post.add_text("Addon to published Post will be refused\n");
    assert_eq!("I ate a salad for lunch today.\nSome more salad…\n", post.content());
}
