fn main() {
    let s1 = String::from("Hello World!!!!!!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);
}
fn calculate_length(s:&String)-> usize{
    // Calculates length of string
    s.len()
}