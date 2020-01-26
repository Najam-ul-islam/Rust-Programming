fn main() {
    let s1 = String::from("hello Rust");
    let hello = &s1[0..5];
    println!("First word of s1: {:?}",hello);
    let rust = &s1[6..10];
    println!("Second word of s1: {:?}",rust);
    let len = s1.len();
    let rust = &s1[6..len];
    println!("{:?}",rust);
}
