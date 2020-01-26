#[derive(Debug)]
// Vectors of different type using enum
enum SpreadSheet{
    Integer(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let vector =vec![SpreadSheet::Integer(3),
                     SpreadSheet::Float(10.04),
                     SpreadSheet::Text(String::from("Hello world"))];

    println!("Enum Method: {:?}",vector.get(0));
    println!("Index Method: {:?}",vector[1]);
}
