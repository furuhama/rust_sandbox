// make Struct

pub fn make_struct() {
    // set Struct instance
    // set `mut` to redefine vars
    let mut u1 = User {
        email: String::from("hoge@example.com"),
        name: String::from("furuhama"),
        sign_in_count: 101,
        active: true,
    };
    println!(
        "name: {}\nemail: {}\ncount: {}\nactive: {}",
        u1.name, u1.email, u1.sign_in_count, u1.active
    );

    // redefine email
    u1.email = String::from("fuga@example.com");
    println!("{}", u1.email);
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
