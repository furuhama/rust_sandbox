// use Enum

#[allow(dead_code)]
pub fn define_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IP v4: {:#?}, IP v6: {:#?}", four, six);

    // use struct
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // try to Option enum

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[allow(dead_code)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}
