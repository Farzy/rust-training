pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
    pub fn stem<'a>(&self, word: &'a str) -> &'a str {
        if word.ends_with(&self.suffix) {
            let index = word
                .rfind(&self.suffix)
                .expect("found because ends_with returned true");
            &word[0..index]
        } else {
            word
        }
    }
}

fn suffix_removal() {
    let stemmer = Stemmer { suffix: String::from("ed") };

    assert_eq!(stemmer.stem("jumped"), "jump");
    assert_eq!(stemmer.stem("jump"), "jump");

    assert_eq!(stemmer.stem("credited"), "credit");
    assert_eq!(stemmer.stem("credit"), "credit");
}

fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        away
    }
}

pub fn main() {
    println!("- Let play a game:");
    let team1 = String::from("Panthers");
    {
        let team2 = String::from("Yellow Jackets");
        let winner = simulate_game(&team1, &team2);
        println!("{} vs {}: {} won", team1, team2, winner);
    }
    println!("Can still discuss {} here.", team1);

    println!("\n- Stemmer");
    suffix_removal();

    let word = String::from("credited");
    let word_stem = {
        let stemmer = Stemmer {
            suffix: String::from("ed"),
        };
        stemmer.stem(&word)
    };
    println!("The stem of {} is {}.", word, word_stem);
}
