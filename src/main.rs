use colored::*;
use mono::repl::start;

fn main() {
    let greeting = "Welcome! This is the Mono (or monkeys, for the friends) programming language."
        .bright_green()
        .bold();
    println!("{}", greeting);
    println!("{}", "\nType some commands:\n".bold());
    start();
}
