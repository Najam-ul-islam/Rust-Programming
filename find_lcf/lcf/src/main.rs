fn main() {
    let result = find_lcf(5, 2);
    println!("LCF is {:#?}",result);
}

fn find_lcf(n1:i64, n2:i64)-> i64 {

let stop = if n1 < n2 {n1 + 1} else {n2 + 1};
for check in 2..stop {
    if (n1 % check == 0) && (n2 % check == 0){
       return check;
    } 
}

0
}