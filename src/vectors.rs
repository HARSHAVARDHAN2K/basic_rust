pub fn main() {
    let mut x = Vec::new();
    let s = 32;
    x.push(s);

    let mut my_vec = vec![1, 2, 3, 4, 5, 6];
    println!("The third number is {}", my_vec[2]);

    for i in my_vec.iter(){
        println!("{}",i);
    }
    my_vec.remove(0);
    for i in my_vec.iter(){
        println!("{}",i);
    }

}
