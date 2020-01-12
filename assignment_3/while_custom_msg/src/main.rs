/*Q # 2. Write a Rust program, use only while loop, print from Number 1 to 10, and make a
         conditional IF for printing a custom message Special Security check , on Number 3, 7 and 10.

*/
fn main() {
    println!("X************************************X");
    println!("\tPrinting Custom Message ");
    println!("X************************************X");
    let mut count = 1;
        while count <= 10{
             
            if count == 3 || count == 7 || count == 10 {println!("Special Security check");}
            else {
                println!("{}", count);
            }
            count+=1;
        }
   
}
