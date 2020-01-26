use std::io;
pub fn example(){
        let list = [1,2,3,4,5,6,7,8,9];
        println!("Enter number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        println!("You enterd: {:?}",input);
        // convert string to integer
        let num:u8 = input.trim().parse().unwrap();
            match num {
                list =>  println!("Correct {:?}",list),
                _ =>  println!("Wrong number"),
            }
}
