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
    struct_with_ref();
    two_different_lifetimes();
    struct_impl();
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

fn struct_with_ref() {
    struct P<'a> {
        q: &'a i32
    }

    let x = 10;
    let s;
    s = P { q: &x };

    assert_eq!(*s.q, 10);
}

fn two_different_lifetimes() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }

    let r = &10;
    let s = S { x: &20, y: &30 };

    println!("{}", sum_r_xy(r, s));
}

fn struct_impl() {
    struct StringTable {
        elements: Vec<String>,
    }

    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    let s = StringTable { elements: vec!["hoge".to_string(), "fuga".to_string(), "piyo".to_string()] };
    let p = &"ho";
    println!("{}", s.find_by_prefix(p).unwrap());
}
