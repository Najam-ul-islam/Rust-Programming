use std::io;
fn main() {
    println!("*************************************");
    println!("\t Question 1 ");
    println!("*************************************");
    let result  = test_result();
    // fail OR pass condition 
    if result >= 70.0 {println!("You are pass with {:#?}% marks.", result);}
    else {println!("You are fail with {:#?}% marks.",result); }
    println!("************************************************");
}
fn test_result()->f64 {
    println!("************************************************");
    // getting input from user
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("success");
    // Subject1 input
    println!("Enter subject1 marks:");
    let mut sub1 = String::new();
    io::stdin().read_line(&mut sub1).expect("success");
    // Subject2 input
    println!("Enter Subject2 marks:");
    let mut sub2 = String::new();
    io::stdin().read_line(&mut sub2).expect("success");
    /***********************************************/
    // converting string to integer
    let subject1_cnvt:f32 = sub1.trim().parse().expect("success");
    let subject2_cnvt:f32 = sub2.trim().parse().expect("success");
    // Percentage formula 
    // into() method convetrs f32 to f64 type
    let percent:f64 = (((subject1_cnvt + subject2_cnvt) * 100.0)/200.0).into();
    println!("percentage is {:#?}%",percent);
    percent
   
    }


