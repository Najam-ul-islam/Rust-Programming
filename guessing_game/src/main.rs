/* 1) Prompt Player to Enter a Guess 
   2) Generate Random Integer Between 1 and 100
   3) Check either  Guess is too low or High 
   4) If Guess is Correct Print Congratulation Message
*/
use std::io; // Standard Library for user Input output.
use rand::Rng; // Random number generation library
fn main() {
    // Prompt Player to Enter a Guess
    println!("Guess The Number!");
    // Generate Random Integer Between 1 and 100
    let random_number = rand::thread_rng().gen_range(1,101);
    loop {
    
    println!("Please guess a number ?");
    println!("Quit a game Type 'x' ");
    let quit = "x";
    let mut guess = String::new(); // Creates instance of empty string
    io::stdin().read_line(&mut guess).expect("Failed to read lines.");
    // Converting String to Integer
    let guess_num:u8 = guess.trim().parse().expect("Input your guess");
    println!("Your guess: {}",guess);
    
   // Check either  Guess is too low or High or equal
   
        if  guess_num < random_number {println!("Too Low");} 
        else if guess_num > random_number{println!("Too High");}
        else if guess_num == random_number{println!("Your guess: {} , Secret Number: {}",guess_num,random_number);break}
        else if guess == quit {break}
        }
   
   }
