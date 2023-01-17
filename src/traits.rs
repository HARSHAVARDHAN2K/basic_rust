/* A trait tells the Rust compiler about functionality a particular type
has and can share with other types. Traits are an abstract definition of
 shared behavior amongst different types. So, we can say that traits are
  to Rust what interfaces are to Java or abstract classes */

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

trait HasVoiceBox {
    //speak
    fn speak(&self);
    //Check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

pub fn main() {
    let harsha = Person {
        name: String::from("Harsha"),
        age: 22,
    };
    println!("{:#?}", harsha.to_string());
    harsha.speak();
    println!("can he speak {}",harsha.can_speak());
}
