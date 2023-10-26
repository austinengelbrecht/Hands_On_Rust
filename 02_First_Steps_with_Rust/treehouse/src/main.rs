#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let visitor_list = [
        Visitor::new("bert", "Hey Bert! enjoy the tree house!"),
        Visitor::new("austin", "Hi Austin! Welcome back!"),
        Visitor::new("fred", "Wow, who invited fred?"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You're not on the list. Please leave..."),
    }

    // let mut let_them_in = false;

    // for visitor in &visitor_list {
    //     if visitor == &name {
    //         let_them_in = true;
    //     }
    // }

    // if let_them_in {
    //     println!("Welcome, to the tree house, {}", name);
    // } else {
    //     println!("Sorry, you're not on the list!");
    // }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}
