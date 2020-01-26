#[derive(Debug)]
struct Student {
    name:String,
    score:u8,

}
/********************************************/
    impl Student{
    fn impl_print(&self){
        println!("{:?}",self.name );
    
    }
    fn impl_ret(&self)-> u8{
        self.score
    }
/********************************************/
}
/********************************************/
fn main() {
/*********************************/
    let student = Student{
        name:"Najam".to_string(),
        score:86,
    };
/*********************************/
    let guest = Student{
        name:"Irfan".to_string(),
        score:86,
    };
/*********************************/
    //println!("{:?}",student);
    sanna(&student);
    guest.impl_print();
  //  guest.impl_ret();
    println!("{}",guest.impl_ret());

}
/********************************************/
/********************************************/
fn sanna(student:&Student) {
    println!("{}",student.name);
}
/********************************************/
