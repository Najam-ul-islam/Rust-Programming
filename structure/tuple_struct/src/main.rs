struct Colour(u8,u8,u8);

fn main() {
    let mut col = Colour(255,100,250);
    // Making Mutable we will add mut keyword before  col variable
    col.0 = 200;
    println!("{}, {}, {}", col.0, col.1, col.2);


}
