// testing references

pub fn references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }
    assert!(*r == 20);
}
