#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}!", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member!", self.name),
            VisitorAction::Refuse => println!("{} is not allowed!", self.name),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Austin", VisitorAction::Accept, 24),
        Visitor::new(
            "Steph",
            VisitorAction::AcceptWithNote {
                note: String::from("Is working on her nails."),
            },
            24,
        ),
        Visitor::new("Aaron", VisitorAction::Refuse, 24),
        Visitor::new("Luke", VisitorAction::Probation, 25),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        // match known_visitor {
        //     Some(visitor) => visitor.greet_visitor(),
        //     None => {
        //         if name.is_empty() {
        //             break;
        //         } else {
        //             println!("{}, is not on the visitor list...", name);
        //             visitor_list.push(Visitor::new(&name, "New Friend!"))
        //         }
        //     }
        // }
    }

    closing_words(visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}

fn closing_words(visitors: Vec<Visitor>) {
    println!("Thanks for visiting the tree house!");
    println!("Here is the guest list:");
    println!("{:?}", visitors);
}
