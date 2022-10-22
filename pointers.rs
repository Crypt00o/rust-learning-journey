
// what is Box ? 
// Box is a smart pointer data type in rust 


fn what_is_box(){
let mut i32_pointer:Box<i32>=Box::new(10);
*i32_pointer=20;
println!("{}",*i32_pointer*2);
}


// createing Smart Pointer like Box From Scratch  


// defineing our struct :CryptoBox  

struct CryptoBox<T>{
    crypto_box_value:T
}

// defineing trait : new 

trait New<T>{
    fn new(value:T)->Self;
}

// defineing trait : Dereference, to dereference the struct CryptoBox to get his value

trait  Dereference{

    // ?Sized mean tell the compiler size of type isn,t known at compile-time
    
    type Target:?Sized;
    
    fn deref(&self)->&Self::Target;

}

// implementing New Trait for CryptoBox

impl <T> New<T> for CryptoBox<T> {
    fn new(value:T)->Self{
        return Self{crypto_box_value:value}
    }
}


// implementing std::ops:Deref for CryptoBox

impl <T>std::ops::Deref for CryptoBox<T> {
    type Target = T;
    fn deref(&self)->&Self::Target {
        return &self.crypto_box_value;
    }
}

// implementing our Dereference trait  for CryptoBox

impl <T> Dereference for CryptoBox<T> {
    type Target = T;
    fn deref(&self)->&Self::Target {
        return &self.crypto_box_value;
    }
}

fn passing_param_useing_impicit_derefference(value:&i32){
println!("passed as parameter useing implicit derefference : {}",value);
}




fn CryptoBox_Test(){
    let mybox:CryptoBox<i32>=CryptoBox::new(33);
    
    // 4 ways to dereference our CryptoBox
    println!("dereference useing * operator : {}",*mybox);
    println!("dereference useing deref(&self)->&Self::Target method : {}",mybox.deref());

    // in the fact useing *struct is shorthand for useing *(struct.deref())

    println!("dereference useing *Dereference::deref(&self)->&Self::Target : {}",*Dereference::deref(&mybox));
    println!("dereference useing shorthand *(mybox.deref()) which equal to use * :  {}",*(mybox.deref()));
    
   // in the fact type of &mybox is &CryptoBox<i32> but it will pass as parameter to fn(&i32)
   // because it implement in this case implicit dereference
     
    passing_param_useing_impicit_derefference(&mybox);

}



fn main(){
   CryptoBox_Test();
}
