// lifetime

use std::fmt::Display;

pub fn lifetime() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    //     // x is dropped here.
    // }
    // // this causes an error
    // println!("{}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // let i = ImportantExcerpt { part: "nyaaaaaaaaaan" };
    // let _ = i.announcement_and_return_part();

    // let string3 = "xyz";
    // let string4 = "xyzhogehoge";
    // longest_with_an_announcement(string3, string4, i);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
