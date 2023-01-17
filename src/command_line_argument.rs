use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    for arguments in args{
        println!("{}",arguments);
    }
}
