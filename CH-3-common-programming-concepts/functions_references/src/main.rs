use typename::TypeName;
fn main() {
    let some_string = String::from("Hello how are you ?");
    function(&some_string);
    function(&some_string);
    function(&some_string);

    procedures(2 ,7.9);
    // function with multiple return values
    let values =  multipule_retruns();
    println!("{:?}",values.0);
    println!("{:?}",values.1);
}
// Functions with retrun
fn function(name: &String) -> &String {
    println!("This function returns String type.");
    // Returning String type
    name
}
// Multiple values returns from function
fn multipule_retruns() -> (i32, f32) {
    println!("This function returns multiple values: ", );
    (-2, 1.345)
}
// Procedures
fn procedures(x:i32, y:f32){
    // Here type-casting i32 to f32 to make addition possible
    // outputs f32 type
     let sum = x as f32 + y;
     println!("Sum is: {:?}",sum.type_name_of() );
     // Outputs the i32 type
     let sum_1 = x + y as i32;
     println!("Sum is: {:?}",sum_1.type_name_of() );
}
