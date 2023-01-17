use std::fs::File;
use std::io::prelude::*;
 
pub fn main() {
    let mut file = File::create("output.txt").expect("Could not create file!");

    file.write_all(b"hi i am harsha").expect("can not write into the file");
}
