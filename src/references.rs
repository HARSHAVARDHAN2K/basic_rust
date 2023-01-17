pub fn main() {
    let mut x = 10;
    x = 15;
    let xr = &x;
    println!("{} and it's reference is {}", x, xr);
{
    let dom = &mut x;
    *dom+=1;
    println!("{}", dom); 
}

println!("{x}");
    

}
