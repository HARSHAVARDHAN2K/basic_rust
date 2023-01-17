struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    let my_rect = Rectangle {
        width: 10,
        height: 15,
    };
    my_rect.print_description();
    let mut x = my_rect.area();
    println!("area of the Rectangle is {}", x);
    if my_rect.is_square() {
        println!("Rectangle is a square");
    } else {
        println!("Rectangle is not a square");
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_description(&self) {
        println!("width: {} height: {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
