#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,

}
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self , other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 70,
    };
    let rect2 = Rectangle{
        width: 40,
        height: 50,
    };
    println!("{:?}",rect1.can_hold(&rect2) );
}
