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

    let i = ImportantExcerpt { part: "nyaaaaaaaaaan" };
    println!("{}", i.level());
    let s3 = String::from("sleepy");
    let _ = i.announcement_and_return_part(s3.as_str());

    let string3 = "xyz";
    let string4 = "xyzhogehoge";
    let disp = 100;
    longest_with_an_announcement(string3, string4, disp);
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

// the explicit lifetime declaration is needed if the struct contains a reference
// this annotation means the instance of this struct could not live longer than the reference
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announcement_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// this is a function with generics, lifetime parameter and bouding trait
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
