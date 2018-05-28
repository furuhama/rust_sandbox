extern crate libc;

extern "C" {
    fn plusone(x: libc::c_int) -> libc::c_int;
}

pub fn run_libc() {
    unsafe {
        let mut x = 0;

        while x < 10 {
            x = plusone(x);
        }

        println!("{}", x);
    }
}
