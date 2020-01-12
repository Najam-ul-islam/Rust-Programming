fn main() {
    let s = String::from("Hello World!!!");
//    takesownership(s)
 //   takes(10)
 let s1 = String::from("Hello");
 //let s2 = s1;
 let s2 = s1.clone();
 println!("s1 = {},s2 = {}",s1 , s2);
/*
{
let student = String::from("Muhammad Najam Ul Islam");
println!("Name {}",student );
}

}

fn takesownership(somestring:String){
    println!("{:?}",somestring)
 
    */
}