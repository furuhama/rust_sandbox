// test Pattern matching

pub fn pattern_matching() {
    let opt: Option<i32> = None;
    let unwrapped = match opt {
        Some(v) => v,
        None => 100,
    };

    println!("the result is: {}", unwrapped);

    conditional_arm();
}

fn conditional_arm() {
    let target = Some(10);
    let _ = match target {
        Some(ref n) if n%10 == 0 => {
            println!("the number can be divided by 10!: {}", n);
        },
        Some(n) => {
            println!("the number: {}", n);
        },
        _ => {},
    };
}
