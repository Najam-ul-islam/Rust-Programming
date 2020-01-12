use std::io;

fn main() {
    // let names_list: &str = "Irfan";
    println!("Enter your name:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name);

    match name {
        "Irfan" => println!("Allowed"),
        _ => println!("Denied")
    }
    }

