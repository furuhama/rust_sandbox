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

    let u2 = build_user(
        String::from("mofumofu@example.com"),
        String::from("furufuru"),
    );
    println!("email: {}, name: {}", u2.email, u2.name);

    // update instance
    let u3 = User {
        email: String::from("neko@example.com"),
        name: String::from("nyanko"),
        ..u2
    };
    println!(
        "name: {}\nemail: {}\ncount: {}\nactive: {}",
        u3.name, u3.email, u3.sign_in_count, u3.active
    );

    // make instance by tuple struct
    let color_test = Color(0, 10, 352);
    println!("{}, {}, {}", color_test.0, color_test.1, color_test.2);
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

// just make struct by tuple
struct Color(i32, i32, i32);
