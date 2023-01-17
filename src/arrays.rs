pub fn arrays() {
    let mut numbers = [1,2,3,4,5,6,7,8,9];
 
    println!("{}",numbers[5]);

    for i in numbers.iter_mut(){
            println!("{}",i);
    }
    println!("{}",numbers[5]);

    let mut num: [&str;4] = ["harsha","vardhan","sow","shashi"]; 
    println!("{:#?}",num);

    let x = [2;5];
    println!("{:#?}",x);
    
}
