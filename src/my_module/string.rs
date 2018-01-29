// String

pub fn string() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let mut s1 = String::from("hoge");
    s1.push_str("fuga");
    println!("{}", s1);

    let mut s2 = String::from("furuhama");
    let s3 = "piyomaru";
    s2.push_str(&s3); // use memory of s2, but literal is s3
    println!("s3 is {}", s3);

    let mut s4 = String::from("くそわろ");
    s4.push('た'); // this should be single quote
    println!("{}", s4);

    let s5 = String::from("hello, ");
    let s6 = String::from("rust");
    let s7 = s5 + &s6;
    println!("{}", s7);

    let s8 = String::from("hoge");
    let s9 = String::from("fuga");
    let s10 = String::from("piyo");
    // let s11 = s8 + "-" + &s9 + "-" + &s10;
    // println!("{}", s11);
    let s12 = format!("{}-{}-{}", s8, s9, s10);
    println!("{}", s12);

    let len = String::from("long goodbye").len();
    println!("{}", len);

    let hello = "Здравствуйте";
    let hello_part = &hello[0..4];
    println!("{}", hello_part);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "にゃーん".bytes() {
        println!("{}", b);
    }
}
