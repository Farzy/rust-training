use std::fmt::Display;

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Copy> Point<T> {
    fn x2(&self) -> T {
        self.x
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary2 for NewsArticle {}

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');

//------------------------------------------------------------

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

//------------------------------------------------------------

    let integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
//    let wont_work = Point { x: 5, y: 4.0 };

    let _both_integer = Point2 { x: 5, y: 10 };
    let _both_float = Point2 { x: 1.0, y: 4.0 };
    let _mixed = Point2 { x: 5, y: 4.0 };

    println!("integer.x = {}", integer.x());

    let s = Point { x: "coucou", y: "test" };
    println!("s.x = {}", s.x());
    println!("s.x (Copy version) = {}", s.x2());
    assert_eq!(s.x(), &s.x2());

    //------------------------------------------------------------

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    //------------------------------------------------------------

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize2());
}

