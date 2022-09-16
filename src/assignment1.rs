// Name: Jenny Nguyen
// Course: CS-420 - Advanced Programming Languages
// Assignment 1 - Rust Increase Function
pub fn increase(number: i64){
    println!("{}", number + 1);
    return number;
}

fn main() {
    increase(10);
}