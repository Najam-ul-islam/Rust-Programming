#[derive(Debug)]
struct Student{
    name: String,
    roll_no: u32,
    grade: char,
    marks: u32,
    average: f32,
}
fn main() {
    let s1 = Student{
        name: String::from("Muhammad Najam Ul Islam"),
        roll_no: 55985,
        marks: 800,
        average: 87.5,
        grade: 'A',
    };
    println!("Name: {:?}",s1.name);
    println!("Roll No: {:?}",s1.roll_no);
    println!("Marks: {:?}",s1.marks);
    println!("Percentage: {:?}",s1.average);
    println!("Grade: {:?}",s1.grade);

}
