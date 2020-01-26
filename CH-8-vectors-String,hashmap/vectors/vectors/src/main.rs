mod example;
fn main() {
    // Creating new empty vectors
    let vector : Vec<i32> = Vec::new();
    println!("{:?}",vector);
    // vec! macro to create initial vector with values
    let vector_1 = vec![1,2,3];
    println!("{:?}",vector_1);
    //==================================================
    // Droping values by using block or scope
    {
    // Updating vector values
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}",v );
    }
    //==================================================
    let v1 = vec![1,2,3,4,5,6,7];
    // Two ways to get elements of vectors
    // 1. get() method
    let first: Option<&i32> = v1.get(3);
    // 2. Indexing method
    let second: &i32 = &v1[2];
    //          OR
    let second  = &v1[2];
    println!("Using Index Method: {:?}\nUsing Enum: {:?}",second,first);
    //==================================================
    example::example();


}
