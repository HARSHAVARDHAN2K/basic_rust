pub fn main() {
    let mut my_string =
        String::from("Terminal will be reused by tasks, press any key to close it.");
    println!("length :{}", my_string.len());
    println!("is string empty {}", my_string.is_empty());

    for i in my_string.split_whitespace() {
        println!("{}", i);
    }

    println!(
        "Does the String contains another string {}",
        my_string.contains("Terminal")
    );

    my_string.push_str("harsha");
    println!("{:?}", my_string);

    //Replace method
    {
        let mut my_string = String::from("Rust is fantasctic");
        my_string.replace("fantastic", "Awesome");
    }

    /* Lines */
    {
        let my_string = String::from("The weather is nice\n can we go outside");
        
        for line in my_string.lines(){
            println!("[{}]",line);
        }
    }

    /* Split */
    {
        let my_string = String::from("Leave +a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{:?}",tokens);
    }

    /* Trim */
    {
        let my_string = String::from("      My name is Harshavardhan  ");
        println!("Before Trim :{}",my_string);
        println!("All Trim :{}",my_string.trim());
    }

    /* Chars */
    {
        let my_string = String::from("Hello i am harshavardhann t k hoe are you   ");
        my_string.trim();
        //Strings can not be indexed in Rust
        //println!("{}",my_string[3]);

        match my_string.chars().nth(4){
            Some(c) => println!("Character @ index 4 {}",c),
            None => println!("none ")
        }
    }
}
