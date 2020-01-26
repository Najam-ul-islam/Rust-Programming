use typename::TypeName;
#[allow(unused_variables)]
fn main() {
    let slice: &str = "Muhammad ";
    let string: String = String::from("Najam-Ul-Islam");
    // convert slice syte data to String type using
    // to_string() method
    let join = slice.to_string();
    println!("Slice to String type {:?}",join.type_name_of() );
    // another way is to convert slice to String
    let string: String = String::from(slice);
    println!("slice to String type {:?}",string.type_name_of() );

    // using deref symbol '&'
    let slice_from_string:String = String::from("hello Rust");
    let string_to_slice : &str = &slice_from_string;
    println!("String to Slice {:?}",string_to_slice.type_name_of() );
    // concatinate two slice type
    // but output will be String type
    let array = ["hello ", "World!!!"].concat();
    println!("{:?}",array);

    let str_1: &str = "hello ";
    let str_2: &str = "Rust!!!";
    // Using format!() macro
    // Converts slice to String type
    let join_str = format!("{} {}", str_1, str_2);
    println!("{:?}",join_str.type_name_of() );
    //==============================================
    // Iterating over chars
    let get_char = &slice_from_string.chars().nth(1);
    println!("{:?}",get_char);

    match get_char{
        Some(c) => println!("{:?}",c),
        None => {}
    }
    // OR
    if let Some(c) = &str_1.chars().nth(2){
        println!("If let: {:?}",c);
    }
    //==============================================



}
