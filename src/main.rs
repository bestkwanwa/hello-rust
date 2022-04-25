use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};
fn main() {
    // demo1
    let stdout = stdout();
    let message = String::from("Hello fellow Rustanceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    // demo2
    println!("Guess the number! Plz input ur guess.");
    let mut guess = String::new();  // 创建了一个可变变量，将新的String实例绑定到变量上
    stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    println!("---------divider---------");
}
