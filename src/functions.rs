pub fn main() {
     print_numbers_to(10);
     if is_even(16){
        println!("It's a Even number");
     }else{
        println!("It's a Odd number");
     }
}

fn print_numbers_to(x:i32){
    for i in 0..x{
        println!("{i}");
    }
}

fn is_even(x:u32) -> bool {
    if x%2 == 0 {
        true
    }else {
        false
    }
}

 

