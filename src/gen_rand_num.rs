extern crate rand;

use rand::Rng;

pub fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("Random number: {}",random_number);

    //Flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(2);
println!("Random Boolean: {}",random_bool);
}
