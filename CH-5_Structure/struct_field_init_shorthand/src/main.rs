struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
fn main() {
    let u_name = String::from("Najam Ul Islam");
    let mail = String::from("najam@email.com");
    build_user(u_name, mail);
    println!("Is active {:#?}",User);
    
}
fn build_user(username:String, email:String)-> User{
    User {
        // Instead of using "email:email" will use
        // init shorthand "email" only
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}