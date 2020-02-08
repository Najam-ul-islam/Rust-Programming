/* 1) Prompt Player to Enter a Guess
   2) Generate Random Integer Between 1 and 100
   3) Check either  Guess is too low or High
   4) If Guess is Correct Print Congratulation Message
*/
use std::io; // Standard Library for user Input output.
use rand::Rng; // Random number generation library
use std::cmp::Ordering;
fn main() {
    // Prompt Player to Enter a Guess
    println!("X==============================X");
    println!("\tGuess The Number!");
    println!("X==============================X\n");
    println!("Please guess a number between 1 and 100:");
    // Calling coresponding functions
    random_number();
    let u_inp = user_input();

    // Checking Guess
    let check_guess = match u_inp.cmp(&random_number()){
        Ordering::Less => println!("Your guess is low!!"),
        Ordering::Greater => println!("Your guess is high!!"),
        Ordering::Equal => println!("You Win!!!!!!"),
    };
}
// Getting user Input
fn user_input(){
    println!("Please guess a number: ");
    let mut guess = String::new(); // Creates instance of empty string
    io::stdin().read_line(&mut guess).expect("Failed to read lines.");
    let input:u8 = guess.trim().parse().expect("Failed to read lines.");
    println!("You guess: {}",input);
    }
// Generating Random Number
fn random_number() {
    // Generate Random Integer Between 1 and 100
    let random_num:u8 = rand::thread_rng().gen_range(1,101);
    //println!("Random Number is: {}",random_num);
    }
