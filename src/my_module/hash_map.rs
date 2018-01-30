// Hash Map

use std::collections::HashMap;

pub fn hash_map() {
    let mut scores = HashMap::new();
    // basic insertion
    scores.insert(String::from("nyan"), 10);
    scores.insert(String::from("piyo"), 50);
    println!("{:?}", scores);
}
