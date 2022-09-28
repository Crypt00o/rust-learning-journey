#![allow(warnings)]

struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn new() -> Self {
        return Self {
            name: String::new(),
            age: 0,
        };
    }

    fn init(name: &str, age: u8) -> Self {
        return Self {
            name: String::from(name),
            age,
        };
    }

    fn clone(&self) -> Self {
        return Self {
            name: self.name.clone(),
            age: self.age,
        };
    }

    fn clear(&mut self) {
        *self = Person::new();
    }

    fn is_equal_to(&self, other: &Self) -> bool {
        if self.name.as_str() == other.name.as_str() && self.age == other.age {
            return true;
        } else {
            return false;
        }
    }

    fn info(&self) {
        println!("name is : {} , age is : {} ", self.name, self.age)
    }
}

fn main() {
    let person: Person = Person::init("eslam", 20);
    let mut person2: Person = person.clone();
    println!(
        "is the 2 persons are equals ? : {}",
        person2.is_equal_to(&person)
    );
    person2.info();
    person2.clear();
    person2.info();
}
