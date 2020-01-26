// line prints whole student1 structure instance
#[derive(Debug)]
struct Student{
    name: String,
    s1: i32,
    s2: i32,
}

fn main() {
    // Instance of Student Structure
    let student1 = Student {
        name: String::from("Muhammad Najam"),
        s1: 80,
        s2: 90
    };

    println!("Name :{}", student1.name);
    println!("Subject 1 Marks:{}", student1.s1);
    println!("Subject 2 Marks:{}", student1.s2);
    println!("Subject 2 Marks:{:?}", student1);

    let mut student2 = Student {
        name: String::from("Muhammad Ali"),
        s1: 70,
        s2: 95,
    };

    println!("Name :{}", student2.name);
    println!("Subject 1 Marks:{}", student2.s1);
    println!("Subject 2 Marks:{}", student2.s2);
    println!("Subject 2 Marks:{:?}", student2);
    student2.s2 = 99;
    println!("Subject 2 Marks:{}", student2.s2);

    let student3 = Student {
        name: String::from("SannaUllah"),
        s1: student2.s1,
        s2: student2.s2,
    };
    println!("Name :{}", student3.name);
    println!("Subject 1 Marks:{}", student3.s1);
    println!("Subject 2 Marks:{}", student3.s2);
    println!("Subject 2 Marks:{:?}", student3);

    let student4 = Student {
        name: String::from("Mazhar"),
        // copy all the field data of student 3
        ..student3
    };
    println!("Name :{}", student4.name);
    println!("Subject 1 Marks:{}", student4.s1);
    println!("Subject 2 Marks:{}", student4.s2);
    println!("Subject 2 Marks:{:#?}", student4);
}

