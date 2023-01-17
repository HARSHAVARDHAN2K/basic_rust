pub fn main() {
    let x = 10;

    //CODE BLOCKS
    /* A block expression, or block, is a control flow expression and anonymous
     namespace scope for items and variable declarations.  */
    {
        let y = 5;
        println!("x : {} y: {}", x, y);
    }
    println!("{}", x);
}
