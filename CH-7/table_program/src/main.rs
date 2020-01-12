//mod lib;
use my_table;
use std::io;
fn main() {
    println!("Enter Number ");
    let mut guess = String::new(); // Creates instance of empty string
    io::stdin().read_line(&mut guess).expect("Failed to read lines.");
    // Converting String to Integer
    let guess_num:u8 = guess.trim().parse().expect("Input your guess");
    println!("Your guess: {}",guess);
    //calling from another file  or programe/module
    //lib::table(guess);
    my_table::table(guess)
}
