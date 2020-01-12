fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;

        }
    };
    println!("The value of counter is {}",result);
    for_loop();
    for_loop_rev();
    while_loop();
    while_array();
}

fn for_loop(){
    
    let a = ["Najam","30","70","88"];
    for element in a.iter(){
        println!("Element  {}",element);
    }

}

fn for_loop_rev(){
    
    for number in (1..4).rev(){
        println!("Number {}",number);
    }

}
//***********************************************
fn while_loop(){
    let mut number = 3;
    while number !=3{
        println!("{}",number);
        number -= 1;
    }
    println!("LIFTOFF!!!!!!");

}
//*********************************************** */
fn while_array(){
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5{
        println!("Value is {} at index  {}",a[index], index);
        index +=1;
    }

}