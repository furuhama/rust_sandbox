// testing references

pub fn references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }
    assert!(*r == 20);

    #[allow(dead_code)]
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 3792 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(rrr.y, 3792);
    assert_eq!(rr.y, 3792);

    test_static();
    test_lifetime();
    return_ref();
}

fn test_static() {
    static mut ST: &i32 = &10;

    fn f(p: &'static i32) {
        unsafe {
            ST = p;
        }
    }

    static CONS: i32 = 100;
    f(&CONS);

    println!("{}", unsafe { ST });
}

fn test_lifetime() {
    fn g<'a>(p: &'a mut i32) {
        *p += 10;
    }

    let mut x = 3;
    g(&mut x);
    println!("{}", x);
}

fn return_ref() {
    let vec = [1, 3, 4, 2, 4, 3, 0, 7, 12];
    let s;
    s = smallest(&vec);

    println!("{}", s);
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}
