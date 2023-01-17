use std::collections::HashMap;

pub fn main() {
let mut x = HashMap::new();     
x.insert("Rust", 96);
x.insert("webdev", 94);
x.insert("solana", 99);

for (i,j) in x.iter(){
    println!("{} => {}",i,j);
}

match x.get("webdev") {
    Some(y) => println!("you got {} for web dev",y),
    None => println!("you didn't study webdev"),
}

//Remove a value
x.remove("Rust");


for (i,j) in x.iter(){
    println!("{} => {}",i,j);
}

}