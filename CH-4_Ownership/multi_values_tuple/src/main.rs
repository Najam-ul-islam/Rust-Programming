fn main() {
    let s1 = String::from("Hello");
    let (s,len) = calculate_length(s1);
    println!("String '{}' length is {}", s, len);
}
fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    // Multiple values returning
    // String , length of that string
    (s, length)
}