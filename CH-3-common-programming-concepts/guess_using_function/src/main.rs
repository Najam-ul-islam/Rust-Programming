/* 1) Prompt Player to Enter a Guess 
   2) Generate Random Integer Between 1 and 100
   3) Check either  Guess is too low or High 
   4) If Guess is Correct Print Congratulation Message
*/
use std::io; // Standard Library for user Input output.
use rand::Rng; // Random number generation library
fn main() {
    // Prompt Player to Enter a Guess
    println!("X==============================X");
    println!("\tGuess The Number!");
    println!("X==============================X\n");
    println!("Please guess a number between 1 and 101:");
    // Calling coresponding functions
    
    user_input();
    random_number();
    check_guess();
}
// Getting user Input
fn user_input(){
    let mut guess = String::new(); // Creates instance of empty string
    io::stdin().read_line(&mut guess).expect("Failed to read lines.");
    println!("You guess: {}",guess);
}
// Generating Random Number
fn random_number(){
    // Generate Random Integer Between 1 and 100
    let _random_num:u8 = rand::thread_rng().gen_range(1,101);
    println!("Random Number is: {}",_random_num);
}
// Checking Guess 
fn check_guess(){
     // Check either  Guess is too low or High 
   if user_input() < random_number(){
        println!("Your guess is too low...\n");
    }
    else if user_input() > random_number() {
        println!("Your guess is too high...\n");
    }
    else if user_input() == random_number(){
        println!("You guess correct number.");
    }
    else {println!("Better luck net time!!!!!");   } 
}

