// generic data types

use std::cmp::PartialOrd;

pub fn generics() {
    // largest function
    let number_list = vec![10, 73, 4, 473, 37, 734, 756];
    let result = largest(&number_list);
    println!("largest number: {}", result);

    let char_list = vec!['y', 'm', 'a', 'z', 'n'];
    let result = largest(&char_list);
    println!("largest char: {}", result);

    // Point Struct
    let integer = Point { x: 1, y: 10 };
    let float = Point { x: 0.5, y: 23.58 };
    let int_and_float = Point { x: 35, y: 0.83 };
    println!("{:?}, {:?}, {:?}", integer, float, int_and_float);
    println!("{}", int_and_float.value_x());

    //
    // Trait
    //
    let tweet = Tweet {
        username: String::from("furuhama"),
        content: String::from("nyaaaaaaaaaaaaaaaaaaaaaaaaan"),
        reply: false,
        retweet: false,
    };
    println!("new tweet!: {}", tweet.summary());

    let news = NewsArticle {
        headline: String::from("BREAKING!"),
        location: String::from("Tokyo"),
        author: String::from("furuhama"),
        content: String::from("nyan nyan"),
    };
    println!("Today's news: {}", news.summary());

    let no_body = NoBody {};
    println!("{}", no_body.summary());
    notify(no_body);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list [0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn value_x(&self) -> &T {
        &self.x
    }
}

trait Summarizable {
    //basic implementations
    // fn summary(&self) -> String;
    fn content(&self) -> String;

    // define default befavior
    fn summary(&self) -> String {
        String::from("404 error...")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn content(&self) -> String {
        format!("{}", self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn content(&self) -> String {
        format!("{}", self.content)
    }
}

struct NoBody {}

impl Summarizable for NoBody {
    fn content(&self) -> String {
        String::from("no content")
    }
}

// function bouding trait
fn notify<T: Summarizable>(item: T) {
    println!("notification: {}", item.summary())
}
