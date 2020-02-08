use std::io;
#[derive(Debug)]
struct BirthDate{
    day: u8,
    month: u8,
    year: u32,
}
fn main() {
    println!("Date of birth should be in DD/MM/YY ");
    println!("Enter day of birth:");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("failed to read day.");
    let p1_day: u8 = day.trim().parse().unwrap();
    //===============================================================
    println!("Enter month of birth:");
    let mut month = String::new();
    io::stdin().read_line(&mut month).expect("failed to read day.");
    let p1_month: u8 = month.trim().parse().unwrap();
    //===============================================================
    println!("Enter year of birth:");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("failed to read day.");
    let p1_year: u32 = year.trim().parse().unwrap();
    //===============================================================
    let person1 = BirthDate{day: p1_day, month: p1_month, year: p1_year,};
    let person2 = BirthDate{day:12, month: 8, year: person1.year,};
    println!("{:#?}",person1);
    //println!("{:#?}",person2);
    println!("Date of birth shuld be in dd/mm/yy format.");
    println!("Date of Birth: {:#?}/{:#?}/{:#?}",person1.day, person1.month, person1.year);
}
