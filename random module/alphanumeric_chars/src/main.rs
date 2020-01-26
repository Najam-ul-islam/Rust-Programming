use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

let mut rng = thread_rng();
let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect();
println!("Random chars: {}", chars);
