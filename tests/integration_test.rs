use hello_world::my_test;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, my_test::add_two(2));
}
