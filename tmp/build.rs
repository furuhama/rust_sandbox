extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/my_module/modules.c")
        .compile("runlibc.a");
}
