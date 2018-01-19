// basic functions

pub fn basic_functions() {
    another_fn();

    fn fn_inside() {
        println!("function inside");
    }
    fn_inside();

    int_fn(100);

    // define var
    let x = 10;

    let y = {
        let x = 2;
        x + 1
    };

    println!("x: {}, y: {}", x, y);

    // fn returning value
    let val = return_value(7);
    println!("{}", val);
}

fn another_fn() {
    println!("hoge fuga piyo");
}

fn int_fn(x: i32) {
    println!("interger: {}", x);
}

fn return_value(x: i32) -> i32 {
    x
}
