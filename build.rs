extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/my_module/c/modules.c")
        .compile("runlibc.a");
}
