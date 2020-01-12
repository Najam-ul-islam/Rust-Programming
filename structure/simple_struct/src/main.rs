// Using Structure to create user defined data
// Color is Structure
struct Color {
    // Members of Color data
    red:u8,
    green:u8,
    blue:u8
}
fn main() {
    let col = Color{red:100, green:200, blue:20};
    // Accessing Color Members using (.) operator
    println!("Red: {}, Green: {}, Blue: {}",col.red, col.green,col.blue);

}
