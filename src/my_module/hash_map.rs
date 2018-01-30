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

    // ownership
    let field_name = String::from("Nyanko");
    let field_value = String::from("hoge");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // can't use field_XXX after insertion
    // println!("{}", field_name);

    // get value from scores(defined above)
    let team_name = String::from("nyan");
    let score = scores.get(&team_name);
    println!("{:?}", score); // returns Some(10)

    // override values
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("nyanko"), 10);
    scores2.insert(String::from("nyanko"), 35);
    println!("{:?}", scores2);

    // only insert if the key has no value
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("hoge"), 10);
    scores3.entry(String::from("fuga")).or_insert(50);
    scores3.entry(String::from("hoge")).or_insert(35); // no changes
    println!("{:?}", scores3);

    // update value by old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
