use std::fs::File;
use std::io::prelude::*;

pub fn main() {

    
    let mut file = File::open("info.txt").expect("can't open file");
    let mut content = String::new();
    //read the data of file to the string
    file.read_to_string(&mut content)
        .expect("Oops! Can not read the file...");

    println!("File Content:\n\n{:?}", content);

    

    
}
