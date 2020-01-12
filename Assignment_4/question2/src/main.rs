/*
Q # 2. Write a rust program, define a function that receives 3 arguments of different data types
(integer, float, boolean) and print them on the console.
*/
fn main() {
    println!("========================");
    println!("     Question # 2");
    println!("========================");

    // Function Calling
    number(10 , 2.3 , true)
}

fn number(int_num:i8, float_num:f32, bool_num: bool ){

    println!("Integer type:-> ({})\nFloat type:->   ({})\nBoolean type:-> ({})", int_num, float_num, bool_num);

    println!("==========x=============");
}