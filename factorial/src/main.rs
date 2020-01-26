use std::io;
fn factorial(){
    println!("Please Enter number to calculate Factorial");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let mut num1:i32=num1.trim().parse().unwrap();
    let mut fact=1;
    let x=num1;
    // while num1>0{  //Using while loop
    //     fact=fact*num1;
    //     num1=num1-1;
    // }
    loop{  //using Loop method
        fact=fact*num1;
        num1= num1-1;
        if num1==0{
            break ;
        }
    }
    println!("The Factorial of {} is: {}",x,fact);
}
fn main() {
    factorial();
}
