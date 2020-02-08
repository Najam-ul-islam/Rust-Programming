#[derive(Debug)]
struct Program<T>{
    a: T,
    b: T,
}
impl<T> Program<T>{
    fn a(&self) -> &T {
        &self.a
    }
    fn b(&self) -> &T {
        &self.b
    }

}
fn main(){
    let p1 = Program{
        a: 10,
        b: 4,
    };
    println!("{:?}",p1.a);
    let add = p1.a + p1.b;
    println!("Sum is: {:?}",add);
}
