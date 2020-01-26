fn main() {
    let x = five();
    let _z = plus_one(5);
    println!("value of x is {}",x);
    // Function Arguments
    another_function(5,6);
    println!("The value of _Z is {}",_z);
}
    // Function parameters
fn another_function(x:i32,y:i32){

    println!("Value of x & y is {} , {}",x , y);
}
// function Return by using ( -> ) arrow
// Must include the type of return value like i32 or any type
fn five() -> i32 {5}
// another example of return value of function
fn plus_one(z:i32) -> i32 { z + 1 }
