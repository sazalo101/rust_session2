use std::io;
fn main() {
    println!("Enter your name");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("Hi,{}",guess);
}
