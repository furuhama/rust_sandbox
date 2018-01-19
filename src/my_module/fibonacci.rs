// return fibonacci number

pub fn get_fibonacci(n: i32) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 0;
    let mut counter: i32 = 1;

    if n == 1 {
        return 1
    }

    loop {
        counter += 1;
        let tmp = a + b;
        b = a;
        a = tmp;

        if counter == n {
            break;
        }
    }
    return a
}

pub fn get_fibonacci_tuple(n: i32) -> i64 {
    let mut tup: (i64, i64) = (1, 0);
    let mut counter: i32 = 1;

    if n == 1 {
        return 1
    }

    loop {
        counter += 1;
        tup = (tup.0 + tup.1, tup.0);

        if counter == n {
            break;
        }
    }
    return tup.0
}

