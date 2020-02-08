// import module to use HashMap
use std::collections::HashMap;

fn main() {
    let mut  team = HashMap::new();
    team.insert("Pak", 200);
    team.insert("Sri", 100);
    team.insert("Ind", 50);
    team.insert("Aus", 20);
    println!("{:#?}",team);

    let t1 = team.get("Pak");
    println!("{:#?}",t1);
    let t2 = team["Aus"];
    println!("{:#?}",t2);

    
}
