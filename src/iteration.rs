pub fn main() {
    let mut n =0;
    //Infinet loop
    loop{
        n+=1;
        if n == 6{
            continue;
        }
        println!("the value of {}",n);
        if n==50 {
            break;
        }
    }

//While loop
    let mut n =1;
    while n<=50 {
        if n%5 == 0{
            println!("{}",n);
        }
        n+=1;
    }

    //For loop
let numbers = 30..51;

for i in numbers{
    if i%10 == 0{
        println!{"the number is {}",i};
    }
}

let mut animals = Vec::new();
animals.push(String::from("Dog"));
animals.push(String::from("Cat"));
animals.push(String::from("Camel"));
animals.push(String::from("squirel"));

println!("The animals are");
for (index,a) in animals.iter().enumerate(){
println!("the animal {} is at index {}",a,index);
}

/* if we did'nt use the iter() then the variable vlues ownership transfered to loop and it dies after ward */

println!("{:#?}",animals);

    
}


