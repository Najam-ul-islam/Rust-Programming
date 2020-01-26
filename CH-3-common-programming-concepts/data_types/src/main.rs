#[allow(unused_variables)]
fn main() {
    // Boolean type
    let boolean :bool = true;
    println!("Boolean type: {:?}",boolean);
    //=================================
    println!("Min range of i8: {}",std::i8::MIN);
    println!("Max range of i8: {}",std::i8::MAX);
    //=================================
    println!("Min range of i16: {}",std::i16::MIN);
    println!("Max range of i16: {}",std::i16::MAX);
    //=================================
    println!("Min range of i32: {}",std::i32::MIN);
    println!("Max range of i32: {}",std::i32::MAX);
    //=================================
    println!("Min range of f32: {}",std::f32::MIN);
    println!("Max range of f32: {}",std::f32::MAX);
    //=================================
    println!("Min range of f64: {}",std::f64::MIN);
    println!("Max range of f64: {}",std::f64::MAX);
    //=================================
    // Tuple
    //=================================
    let x : (i32,f64,u8) = (500,6.9,1);
    // Destructuring the tuple
    let (i , j , k) = x;

    println!("Value of i, j, k is {} , {} and  {}",i,j,k);

    // Accessing tuple by element index number
    // To access element by index (.) Dot operator will be used
    let five = x.0;
    let six_point = x.1;
    let one  = x.2;

    println!("First value is {}",five);
    println!("Second value is {}",six_point);
    println!("Third value is {}",one);
    //==================================
    // Arrays
    //==================================
    let _months =["January", "February", "March", "April", "May", "June",
                 "July", "August", "September", "October", "November", "December"];


  //let months: [i32 ; 5] = [1,2,3,4,5];
    //    println!("Month is {:#?}",months);
        println!("Month is {:?}",_months);
    // Initializing Same element in an array
  /*
  let a = [3;10];
        println!("Elements of Array are {:?}",a);
    // Accessing arrays elements by indexing
    let first = a[0];
    let second = a[1];
    println!(" Elements of array are {:?} & {:?}",first , second);
    let index = 9;
    println!(" {:?}",a[index]);
*/
}
