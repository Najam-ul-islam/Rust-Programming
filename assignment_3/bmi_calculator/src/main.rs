/*Q # 1 . BMI Calculator : Write a Rust program, that get input height (in cm) and weight (in kg)
          from user to calculate body mass index. Use if condition to check if the BMI is <18.5, print
          underweight, if BMI is >18.5 and <25, print Normal weight, if BMI is >25 print overweight.
*/
use std::io;
fn main() {
    println!("X**************************X");
    println!(" Body Mass Index Calculator");
    println!("X**************************X");
// Input from User For weight
println!("Enter weight(Kg):");
let mut weight = String::new();
io::stdin().read_line(&mut weight).expect("Muhammad Najam Ul Islam");
// String to Floating-point conversion
let weight:f32 = weight.trim().parse().expect("PIAIC55985"); 
// Input from User For Height
println!("Enter Height(cm):");
let mut height = String::new();
io::stdin().read_line(&mut height).expect("najam.capricon88@gmail.com");
// String to Floating-point conversion
let height:f32 = height.trim().parse().expect("PIAIC55985");
// BMI  Formula 
let  bmi:f32 = weight / ((height*12.0*2.54/100.0)*(height*12.0*2.54/100.0));
println!("Body Mass Index(BMI) is: {:?}",bmi);
// Condition Check
if bmi < 18.5{println!("Underweight");}
else if bmi > 18.5 && bmi < 25.0{println!("Normal weight");}
else if bmi > 25.0{println!("Over Weight");}
println!("**************************************************");


}
