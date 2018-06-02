// try to extend str type in Rust

pub fn monkey_patch() {
    pub trait StrMixin {
        fn hoge(&self);
    }

    impl StrMixin for str {
        fn hoge(&self) {
            println!("HOGE!");
        }
    }

    "this is str type value".hoge();
}
