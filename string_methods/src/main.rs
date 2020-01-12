
fn main() {
    println!("************************");
    println!("\tString Methods");
    println!("************************");
    {
        // replace() method
        let my_string = String::from("hello 'World'");
        println!("Before replacing string: {}", my_string);
        println!("After replacing string: {}", my_string.replace("World", "Rust"));
    }
//*************************************************************************************
    {
        // lines() Method
        let new_string = String::from("Hello\nRust\nlearner");
        for line in new_string.lines() { println!("[{}]", line); }
    }
//*************************************************************************************
    {
        // split() Method
        let split_str = String::from("I,am,new,to,Rust");
        let tokens: Vec<&str> = split_str.split(",").collect();
        println!("At index 2 value is: {:#}", tokens[4]);
    }
//*************************************************************************************
    {
        // trim() Method
        let trim_str = String::from("   I am new to Rust    \n");
        println!("Before Trim: {}", trim_str);
        println!("After Trim: {}", trim_str.trim());
    }
//*************************************************************************************
    {
        // char() Method
        let char_str = String::from("I am new to Rust");
        println!("Character at {}", char_str);
        // Get character at index[]
        match char_str.chars().nth(5) {
            Some(_c) => println!("Character at index 5: {}", _c),
            None => println!("No character at index 5")
        }
    }
//*************************************************************************************
}
