// Optionals in rust mean option which mean it can have a value or have a no value 

//if it have a value it represent with Some(value).
//if it don,t have a value it represent with None.


fn main(){

    let my_name:Option<&str>=Some("0xCrypt00o");
    let none_name:Option<&str>=None;
    

    // unwarping mean get the value which Option carry like  

    // let me: Optional<&str>=Some("eslam") unwarping mean get &str value which is "eslam"

    //types of unwarping options :

    // 1)- Safe unwarping and getting value of options useing matching


    match my_name {
        Some(value)=>{println!("safe warping value : {}",value)},
        None=>{println!("this option hasn,t any value")}
    }

    //2) - unsafe unwraping : if the value is Some(value) it will work fine , if None it will through a panic
    
    //1) with option.expect("panic msg")

      println!( "this is my name : {} ",my_name.expect("i expect string slice"));

    //this will  through a panic  because none_name option =None mean it hasn,t any value 
   
      println!( "this is my name : {} ",none_name.expect("i expect string slice")); 


    //2) with option.unwarp()
    
    println!( "this is my name : {} ",none_name.unwrap()); //this will  through a panic because none_name option =None mean it hasn,t any value 

 
    //unwarping mutable option with as_mut(&self)

    let mut my_age:Option<u8>=Some(20);
    
    match my_age.as_mut() {
        Some(value)=>*value=*value+1,
        None=>{}
    }

    println!("{}",my_age.unwrap());
    
    
    



}
