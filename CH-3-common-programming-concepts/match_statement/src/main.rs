//use std::io;
//use rand::Rng;
//use typename::TypeName;
fn main() {
    let mut vector:Vec<i32> = Vec::new();
    for codes in 1..100{
        vector.push(codes);
    }
    println!("{:?}",vector);

    let mut i = 0;
    while i < vector.len(){
        if i % 2 == 0{
            println!("{:?}",i);
        }
    i +=1;
    }

    }

    /*
    println!("Enter your choise: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input..");
    println!("your choise: {}", input);
*/
//let compare = "X".to_string();
//println!("{:?}",choise(input).type_name_of() );
