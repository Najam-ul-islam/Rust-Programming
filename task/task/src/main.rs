use std::io;
fn main() {
    let (name,len) = full_name();
    println!("Your Name is:{}\nlength is: {}",name.to_uppercase(),len);
    // Splits String
    let tokens: Vec<&str> = name.split(" ").collect();
    println!("Middle name{:#?}",tokens[0]);
    println!("Last name{:#?}",tokens[1]);
}
fn full_name()->(String,usize) {
    println!("Enter your full name: ");
    let mut getname = String::new();
    io::stdin().read_line(&mut getname).expect("failed to read input.");
    let length = getname.len();
    (getname, length)

}