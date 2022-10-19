//  traits are functionality can be shared and use for more than struct or enum 

struct Person<'l1>{
    name:&'l1 str,
    age:u8
}

struct Animal<'l1>{
name:&'l1 str
}

trait NameLength {
    fn name_len(&self)->usize;
}

trait NewName<'l1> {
    fn new(name:&'l1 str,age:u8)->Self;
}


impl <'l1>NameLength for Person<'l1> {
    fn name_len(&self)->usize {
        return self.name.len();
    }
}

impl <'l1> NewName<'l1> for Person<'l1>{
    fn new(name:&'l1 str,age:u8)-> Self{
        return Self {name,age};
    }
}

impl std::fmt::Display for Person{

}


fn main(){
let person=Person::new("eslam", 20);
println!("length of name : {} is {}",person.name , person.name_len());

}
