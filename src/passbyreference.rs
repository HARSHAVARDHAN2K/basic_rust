#[derive(Debug)]
struct Color{
    red:u8,
    green:u8,
    blue:u8
}

pub fn main() {
     let mut blue = Color{red:0,green:0,blue:255};
     print_color(&mut blue);

}

fn print_color(c:&Color){
    println!("{:#?}",c);
}
   