struct Colour{
    red:u8,
    green:u8,
    blue:u8
}
fn main() {
    let rgb = Colour{red:255, green:50, blue:35};
    // Function Call by reference
    print_colour(&rgb)
}

fn print_colour(col: &Colour){
    println!("{}, {}, {}",col.red,col.green,col.blue);

}
