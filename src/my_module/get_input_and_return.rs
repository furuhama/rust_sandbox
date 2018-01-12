use std::io;

pub fn get_and_print_input() {
    println!("please input some words");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess) // io::stdin().read_line(hoge) は io::Result インスタンス
        .expect("failed to read line.");
        // expect は io::Result が err のときにその命令を停止して、引数の文字列を標準出力から返す
    println!("your input: {}", guess);
}

