/* Shadowing allows you to re-declare a variable in the same scope, using
the same name. The re-declared variable differs from the original by having
 a different type. This is especially useful upon casting data from one
  type into another. */
pub fn main() {
    let mut x = 10;
    {
        //Shadowing
        let x = "harsha";
        println!("{}", x);
    }
    println!("x is {}", x);
}
