#[derive(Debug)]
struct Food{
    name: String,
    price:u16,
    serving:u8,
}
/************************************************/
impl Food{
    // Associeted function
    fn food_inst(n:String,p:u16,s:u8)->Food{
        Food{
            name:n,
            price:p,
            serving:s
        }
    
    }
}
/************************************************/
impl Food{
    // Associeted function
    fn food_inst2(){
       println!("Welcome to me.");
    }
}
/************************************************/
fn main() {
   let name = "Mango".to_string();
   let price = 100;
   let serving = 50;

   let calll = food(name,price,serving);
   println!("{:#?}",calll);
   let f1 = Food::food_inst(name,price,serving);
   
 //  let f2 = Food::food(name,price,serving);
   println!("{:#?}",f1);
   Food::food_inst2();
}

fn food(n:String,p:u16,s:u8)->Food{
    Food{
        name:n,
        price:p,
        serving:s
    }

}

