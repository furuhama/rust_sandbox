// Table type which uses references

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

pub fn table() {
    let mut table = Table::new();

    table.insert(
        "Haruki Murakami".to_string(),
        vec![
            "World's End and Hard-boiled Wonderland".to_string(),
            "Adventure for The Sheep".to_string(),
        ],
    );

    table.insert(
        "Hiroshi Mori".to_string(),
        vec!["Sky Crawlers".to_string(), "S&M series".to_string()],
    );

    show(&table);
}
