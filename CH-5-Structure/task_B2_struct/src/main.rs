use std::io;
#[derive(Debug)]
#[warn(non_snake_case)]
struct Student{
    name:String,
    age:u8,
    grade:char,
    percentage:(f32,f32,f32),
}
// Creating method
impl Student {
    // Associated function
    fn new(&self) -> f32 {
        (self.percentage.0 / self.percentage.1)* self.percentage.2
    }
}

fn main() {
    let stud = Student{
        name:String::from("Najam"),
        age:31,
        grade:'A',
        percentage:(750.0,1033.0,100.0),
    };
    //=========================================================
    println!("Enter a first float number: ");
    let mut input1:String = String::new();
    io::stdin().read_line(&mut input1).expect("Number must be float type");
    let input_1:f32 = input1.trim().parse().unwrap();
    //===================================================
    println!("Enter a second float number: ");
    let mut input2:String = String::new();
    io::stdin().read_line(&mut input2).expect("Number must be float type");
    let input_2:f32 = input2.trim().parse().unwrap();

println!("Percentage: {:?}%",stud.new());
}
