/*
Q # 1. Write a rust program, define a function that receives one argument of any suitable data
type and print whether the number is positive, negative or equal to zero. (hint: if/else)
*/

fn main() {
    println!("========================");
    println!("     Question # 1");
    println!("========================");

    // Function Calling
    number(250)  
  
}

fn number(num:i32){
    
    if num == 0 {println!("Number is Zero: {}",num);}
    else if num > 0 {println!("Number is positive: {}",num);}
    else {println!("Number is Negative: {}",num);}
    
    println!("==========x=============");
    
}