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

    let s1 = String::from("this is longer");
    {
        let s2 = String::from("shorter");
        let r = longest(s1.as_str(), s2.as_str());
        println!("The longest string is: {}", r);
    }

    // let i = ImportantExcerpt { part: "nyaaaaaaaaaan" };
    // let _ = i.announcement_and_return_part();

    // let string3 = "xyz";
    // let string4 = "xyzhogehoge";
    // longest_with_an_announcement(string3, string4, i);
}

// there are four lifetime annotations 'a
// the first 'a defines a lifetime 'a
// the second and third 'a mean the lifetime of each argument is 'a
// the fourth 'a means the lifetime of returned value from this function is 'a
//
// there is a possibility of the returned value lifetime
// if the function has two or more reference arguments
// and the compiler cannot find out which is the correct lifetime of returned value
// therefore, the explicit lifetime declaration is needed
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
