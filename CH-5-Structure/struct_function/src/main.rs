#[derive(Debug)]
struct Student{
    name:String,
    subject1:i32,
}
fn main() {
    let name =String::from("hello");
    let s1 = 32;
    let s2 = 50;
    let user1 = Student{
        name:String::from("Najam ul islam"),
        subject1:90,
    };
    let user2 = Student{
        name:String::from("ali"),
        subject1:user1.subject1,
    };
    structure(user1);
    return_struct(user2);

    let  stu = (String::from("najam"),80,90);
    tuple(stu);
//    let result = fun();
//    println!("{}",result);
}
fn fun(name:String, s1: i32, s2: i32){

}
fn tuple(stu:(String,i32,i32)){
    println!("{}, {}, {}",stu.0,stu.1,stu.2);
}
fn structure(stud10:Student){
    println!("{:?}",stud10.name);
}
fn return_struct(stud11:Student)-> Student{
    println!("{:?}",stud11.name);
    println!("{:?}",stud11.subject1);
    stud11
}