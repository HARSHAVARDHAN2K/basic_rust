#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn main() {
    let mut bg = Color {
        red: 255,
        blue: 0,
        green: 0,
    };

    bg.blue = 45;

    println!("{:#?}", bg);
    println!("{} {} {}", bg.red, bg.blue, bg.green);

    let mut red = Color1(255,0,0);
    red.2 =55;
    println!("red = {:?}",red);
}

//Tuple struct-structs which are haveing only datatypes no variable names
#[derive(Debug)]
struct Color1(u8, u8, u8);
