// return fibonacci number

pub fn get_fibonacci(n: i32) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 0;
    let mut tmp: u64 = 0;
    let mut counter: i32 = 1;

    if n == 1 {
        return 1
    }

    loop {
        counter += 1;
        tmp = a + b;
        b = a;
        a = tmp;

        if counter == n {
            break;
        }
    }
    return a
}

