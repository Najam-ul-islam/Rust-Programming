fn main() {
    {
    // push_str Method
    let mut s = String::from("Hello");
    // push_str Method to append in String
    s.push_str(", World!!!");
    println!("{}", s);
    }
    {
     // Deep copy
     // Using clone() method
     let s1 = String::from("Hello World!!!!");
     let s2 = s1.clone();
        println!("s1 = {}, s2 = {}",s1,s2);
    }


}
