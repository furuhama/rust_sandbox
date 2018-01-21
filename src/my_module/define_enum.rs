// use Enum

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

pub fn define_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IP v4: {:#?}, IP v6: {:#?}", four, six);
}
