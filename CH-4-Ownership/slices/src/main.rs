fn main() {
    let mut s1 =String::from("hello world of rust");
    let word = first_word(&s1);
    println!("Before Clear() Method: {:?}",word);
    // .clear() method will empties the string and
    // making it equal to (" ") space
    s1.clear();
    println!("After Clear() method s1 give: {:?}",s1);
}
fn first_word(s: &String)-> usize {
    // as_bytes() method will convert String to Array of bytes
    let bytes = s.as_bytes();
    // iterator to loop through over array of bytes
    for (i, &item) in bytes.iter().enumerate(){
        // if space is found then return position of that space
        if item == b' ' {
            return i;
        }
    }
    // other wise return length of string
    s.len()
}