fn main() {
    let s = String::new();
    let s = "Hello";
    let s1 = s.to_string();
    println!("{:?}",s1 );
//========================================
    let string = String::new();
    let s2 = "Hello world".to_string();
    println!("{:?}",s2);
//========================================
    let string_1 = String::from("hello rust");
    println!("{:?}",string_1 );
//========================================
    let mut string_2 = String::from("foo");
    // String method " push_str()" to append string slice
    string_2.push_str(" bar");
    // String method "push()" to append single character
    string_2.push('D');
    println!("{:?}",string_2);

// String Concatination
    let str1 = String::from("Hello");
    let str2 = String::from(" World");
    let str3 = str1 + &str2 ;
    println!("{}",str3);
// Multiple Strings Concatination
    let string1 = String::from("Tic");
    let string2 = String::from("Tac");
    let string3 = String::from("Toe");
    let string4 = string1 + "-" + &string2 + "-" + &string3;
    println!("Using + operator: {:?}",string4);
//========================================
    // Using foramt!() macro
    let string5 = String::from("Tic");
    let string6 = String::from("Tac");
    let string7 = String::from("Toe");
    let string8 = format!("{}-{}-{}",string5,string6,string7);
    println!("Using format!() macro: {:?}",string8);
    let s = String::from("hello").len();
    println!("lenght: {:?}",s);



}
