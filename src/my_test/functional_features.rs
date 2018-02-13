// tests for my_module::functional_features()

use my_module;

#[test]
fn call_with_different_values() {
    let mut c = my_module::Cacher::new(|x| x);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
