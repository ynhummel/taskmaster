use std::env;

fn main() {
    println!("Welcome to TaskMaster, your personal tasklist.");
    for argument in env::args() {
        println!("{}", argument);
    }
}
