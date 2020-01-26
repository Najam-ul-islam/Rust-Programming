use std::io;
use std::fs;
fn main() {
    let data = read_username_from_file();
    //println!("{:#?}",data);
    for i in data{
        i.split_whitespace();
        println!("{:?}",i.trim_end());
    }

}
fn read_username_from_file() -> Result<String, io::Error> {
fs::read_to_string("hello.txt")
}
