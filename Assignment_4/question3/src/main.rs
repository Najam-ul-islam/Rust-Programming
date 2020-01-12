/*
Q # 3. Write a rust program, define a function that receives a number and return the number
itself and its square by using tuple.
*/
fn main() {
    println!("========================");
    println!("     Question # 3");
    println!("========================");

    let (square_n1, square_n2) = number_square(2,6);
    print!("Square is ({},",square_n1);
    println!("{})",square_n2);

    println!("==========x=============");
}

fn number_square(_num1:u32, _num2:u32 ) -> (u32, u32) { 
    
    // Enclosing return values in tuple
    (_num1.pow(2), _num2.pow(2))
}

