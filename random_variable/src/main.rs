use rand::Rng;
fn main() {
	println!("Random Number Generator");
	let random_num = rand::thread_rng().gen_range(1,101);
	println!("Random Number is: {}",random_num);

}
