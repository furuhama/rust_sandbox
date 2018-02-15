// tests for my_module::functional_features()

#[cfg(test)]
mod test {
    use my_module;

    #[test]
    fn call_with_different_values() {
        let mut c = my_module::Cacher::new(|x| x);

        // this test occurs an error because of multiple assignment
        // let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn capture_outside_closure() {
        let x = 4;

        // capture x (outside closure)
        // function can't do such thing
        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }

    #[test]
    fn use_move() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        // this test occurs an error because of this line
        // println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        // this returns true, so I think iterator can do next() to unlimit extent
        assert_eq!(v1_iter.next(), None);
    }
}
