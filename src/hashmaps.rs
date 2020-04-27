use std::collections::HashMap;
use rand::Rng;

pub fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores = {:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    println!("Zip = {:?}", teams.iter().zip(initial_scores.iter()));
    let scores : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores zip = {:?}", scores);
    println!("teams = {:?}", teams);

    let a = [1, 2, 3];
    let inverse : Vec<_> = a.iter()
        .rev()
        .map(|&x| 1.0 / f64::from(x))
        .collect();

    println!("rev.inverse({:?}) = {:?}", a, inverse);

    let sc : Vec<_> = scores.iter()
        .map(|(&x,&y)| {println!("Map {} -> {}", x, y); (format!("{}'", x), *y+10) })
        .collect();
    for (x, y) in sc {
        println!("For {} -> {}", x, y);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
//    println!("{} = {}", field_name, field_value);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("get score as option: {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Score after double insert: {:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Scores after or_insert{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 book_reviews.len());
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
//        42
        rand::thread_rng().gen_range(1, 101)
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("defence").or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();


    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        /// Creates a new Viking.
        fn new(name: &str, country: &str) -> Viking {
            Viking { name: name.to_string(), country: country.to_string() }
        }
    }

    // Use a HashMap to store the vikings' health points.
    let mut vikings = HashMap::new();

    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }

}
