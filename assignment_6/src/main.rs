#[derive(Debug)]
struct UserInfo{
    name:String,
    batch:u8,
    reg_id:u32,
}
fn main() {
    println!("****************************************************");
    println!("\tAssignment 6");
    println!("****************************************************");
    // Structure Instance
    /****************************************************/
    let user1 = UserInfo{
        name:String::from("Muhammad Najam Ul Islam"),
        batch:3,
        reg_id:55985,
    };
    /****************************************************/
    /****************************************************/
    // 2nd Instance
    let user2 = UserInfo{
        name:String::from("2nd Instance "),
        batch:3,
        reg_id:55985,
    };
    /****************************************************/
    println!("User Information:{:#?}",user1);
    println!("Name:{}",user1.name);
    println!("Batch:{}",user1.batch);
    println!("Roll Number:{}",user1.reg_id);
    println!("****************************************************");
    /***************************************************/
    // Calling user2 instance
    let result = student(user2);
    // complete instance printing
    println!("{:#?}",result);
    // Printing by fields
    println!("**************************");
    println!("Name:{:#?}",result.name);
    println!("Batch:{:#?}",result.batch);
    println!("Reg ID:{:#?}",result.reg_id);
    println!("**************************");
    println!("****************************************************");
}
fn student(ret_user2:UserInfo)->UserInfo{
    UserInfo{
        name:ret_user2.name,
        batch:ret_user2.batch,
        reg_id:ret_user2.reg_id,
    }
}