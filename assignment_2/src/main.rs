fn main() {
    /* Q # 1 . Write a Rust program to perform mathematical operations between two numbers.
               Declare two integer variables and assign some values to them. Then print the result of addition,
               subtraction, division, and multiplication in between these two variables.*/

    let num_1:i32 = 10;
    let num_2:i32 = 20;
    let sum = num_1 + num_2 ;       // Addition
    let multiply = num_1 * num_2;  //  Multiplication
    let division = num_2 / num_1;  //  Division
    let subtract = num_2 - num_1;  //  Subtraction
    let remainder = num_1 % num_2; //  Remainder
    println!("Addition is :{}\nMultiplication is :{}\nDivision is :{}\nSubtraction is :{}\nRemainder is :{}\n",
            sum,multiply,division,subtract,remainder);
    println!("*******************************************");
    /* Q # 2. Write a Rust program and declare a tuple for data of a Fruit name, its weight and its
              price. Destructure the tuple in separate variables and print them on your screen/terminal.*/

    let fruits  = ("Mango" , 10, 150.45); // Fruit name , Weight , Price
    // Destructuring via Seperate variables 
    let  (fruit_name , weight , price) = fruits ;        
    println!("Fruit is {} ",             fruit_name);
    println!("Weight is {} Kg",          weight);
    println!("Price of fruit is {} PKR", price);
    println!("*******************************************");
    // Destructuring via Pattren Matching
    let fruit_name = fruits.0;
    let weight = fruits.1;
    let price = fruits.2;
    println!("Fruit is {} ",fruit_name);
    println!("Weight is {} Kg",weight);
    println!("Price of fruit is {} PKR",price);
    println!("*******************************************");
    /* Q # 3 . Write a Rust program by initializing an array of cricket team’s names, and another array
               with their year of winning the world cup. Print the data as below: 
                Output be like:
                Cricket Team: Pakistan - Year: 1992
                Cricket Team: SriLanka - Year: 1996
        */
    let cricket_teams =["Pakistan","Srilanka","Australia","India","England"];
    let winning_year = [1992,1996,1999,2011,2019];
    println!("Cricket Team: {} -   Year {}",cricket_teams[0] , winning_year[0]);
    println!("Cricket Team: {} -   Year {}",cricket_teams[1] , winning_year[1]);
    println!("Cricket Team: {}-   Year {}",cricket_teams[2] , winning_year[2]);
    println!("Cricket Team: {}    -   Year {}",cricket_teams[3] , winning_year[3]);
    println!("Cricket Team: {}  -   Year {}",cricket_teams[4] , winning_year[4]);
    println!("*********************************************");

}
