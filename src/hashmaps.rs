pub fn main() {
    use std::collections::HashMap;

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
}
