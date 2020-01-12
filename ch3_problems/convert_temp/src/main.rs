fn main() {
   // This programe converts temperature 
   celsius_to_fahrenheit();
   fahrenheit_to_celsius();
}
fn celsius_to_fahrenheit(){
    let temp_celsius = 50;
    let to_fahrenheit = temp_celsius * (9/5)+ 32 ;
    println!("Temprature in Fahrenheit: {}", to_fahrenheit); 
}

fn fahrenheit_to_celsius(){
    let temp_fahrenheit = 122;
    let to_celsius = (temp_fahrenheit - 32 ) * (9/5);
    println!("Temprature in Celsius:    {}", to_celsius);
}