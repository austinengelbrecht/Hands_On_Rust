#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let visitor_list = ["bert", "steve", "fred"];
    let mut let_them_in = false;

    for visitor in &visitor_list {
        if visitor == &name {
            let_them_in = true;
        }
    }

    if let_them_in {
        println!("Welcome!");
    } else {
        println!("Sorry, you're not on the list!");
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}
