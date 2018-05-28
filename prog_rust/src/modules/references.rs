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
}
