use std::io;
use std::cmp::Ordering;
fn main() {
    let _months:[u8;12] = [31,28,31,30,31,30,31,31,30,31,30,31];    
    month();
    days();
}

fn month(){
    println!("Enter Month number:");
    let mut month = String::new();
    io::stdin().read_line(&mut month).expect("Sucess");
    // Convert from string type to integer type
    let month_to_num :u8 = month.trim().parse().expect("Error");
    println!("Enterd Month Number:{:#?}",month_to_num);
}
fn days(){
    println!("Enter day number:");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Sucess");
    // Convert from string type to integer type
    let day_to_num :u8 = day.trim().parse().expect("Error");
    println!("Enterd day Number:{:#?}",day_to_num);
}
