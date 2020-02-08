use std::io;
fn main(){
    loop{
    let mut empty_vec: Vec<f32> = Vec::new();
    println!("Input any number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let  num:f32 = input.trim().parse().expect("Invalid Input!!!!!");

    let vector = empty_vec.push(num);
    println!("{:#?}",vector);
/*
    for i in 0..10{
        empty_vec.push(num);
        if i => 10{
            break
        }
        else{
            continue
        }
    println!("{:?}", );
    }
*/
    }
}
