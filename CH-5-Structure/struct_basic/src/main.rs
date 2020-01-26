// Defining a Structure
#[derive(Debug)]
struct UserInfo{
    // fields of structure user_info
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
// Creating an instance of user_info struct
    let user1 = UserInfo {
        email: String::from("najam.capricon88@gmail.com"),
        username: String::from("Najam ul islam"),
        active: true,
        sign_in_count: 1,
};
    let user2 = UserInfo{
      email:String::from("user2@email.com"),
      username:String::from("Muhammad Ali"),
      active:true,
      sign_in_count:user1.sign_in_count,
    };
    println!("User Info:{:#?}",user1);
    println!("username :{}",user1.username);
    println!("Email :{}",user1.email);
    println!("Is active :{}",user1.active);
    println!("Sign-in Tries :{}",user1.sign_in_count);

    println!("User Info:{:#?}",user2);
    println!("username :{}",user2.username);
    println!("Email :{}",user2.email);
    println!("Is active :{}",user2.active);
    println!("Sign-in Tries :{}",user2.sign_in_count);
}
