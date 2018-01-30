// Hash Map

use std::collections::HashMap;

pub fn hash_map() {
    let mut scores = HashMap::new();
    // basic insertion
    scores.insert(String::from("nyan"), 10);
    scores.insert(String::from("piyo"), 50);
    println!("{:?}", scores);

    // set keys and values as vector first, combine them
    let teams = vec![String::from("nyan"), String::from("piyo")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}
