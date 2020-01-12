fn main() {
    let s = String::from("hello RUST");
    takes_ownership(s); // 's' moved to function

    let x = 5;
    makes_copy(x);         // 'x' copy because of i32 type

}   // scope ends here but both s and x moves so nothing happens
fn takes_ownership(from_string: String){    // from_string comes into scope
    println!("String: {}",from_string);
}   // from_string "Drop" here
fn makes_copy(integer: i32){  // integer comes into scope
    println!("Integer value: {}",integer);

}   // Nothing will happen  because x was "Copy"

