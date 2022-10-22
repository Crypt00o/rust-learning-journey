//  traits are functionality can be shared and use for more than struct or enum 

//creating structs to implement traits for it

struct Person<'l1>{
    name:&'l1 str,
    age:u8
}

// createing 3 traits 

trait NameLength {
    fn name_len(&self)->usize;
}

trait PrintHello {
    fn hello(&self)->();
}

trait NewName<'l1> {
    fn new(name:&'l1 str,age:u8)->Self;
}

// implementing traits for structs

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


impl <'l1> PrintHello for Person<'l1>{
    fn hello(&self)->() {
        println!("hello"); 
    }
}

//traits as parameters 

fn trait_as_patameter(some_trait:&impl NameLength){
    println!("{}",some_trait.name_len());
}

// trait as generic parameter

fn trait_as_generic<T>(some_trait:&T) where T:NameLength{
     println!("{}",some_trait.name_len());
}

// multiable traits as  one generic parameter

fn multi_trait_as_one_generic<T>(some_trait:&T) where T:NameLength+PrintHello{
    some_trait.hello();
}

// useing where to share functionality from trait to another trait like ( implementing trait for another trait 

trait IamTraitWillImplementedFromAnotherTrait where Self:PrintHello{
    fn iam_trait_implement_from_another_trait(&self)->();
}

impl <'l1>IamTraitWillImplementedFromAnotherTrait for Person<'l1> {
    fn iam_trait_implement_from_another_trait(&self)->(){ 
         println!("iam trait implemented for another trait");
         self.hello(); 
    }

}


fn main(){
let person=Person::new("eslam", 20);
println!("length of name : {} is {}",person.name , person.name_len());
trait_as_patameter(&person);
trait_as_generic(&person);
person.iam_trait_implement_from_another_trait();
}
