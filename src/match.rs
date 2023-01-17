pub fn main() {
    let number = 11;

    match number {
        1 => println!("it is 1"),
        2..=20 => println!("number between 2 and 20"),
        _ => println!("it dose not match"),
    }
}
