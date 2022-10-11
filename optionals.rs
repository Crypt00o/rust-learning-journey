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


    //2) with option.unwarp() , force unwraping unsafely
    
    println!( "this is my name : {} ",none_name.unwrap()); //this will  through a panic because none_name option =None mean it hasn,t any value 

 
    //unwarping mutable option with as_mut(&self)

    let mut my_age:Option<u8>=Some(20);
    
    match my_age.as_mut() {
        Some(value)=>*value=*value+1,
        None=>{}
    }

    println!("{}",my_age.unwrap());
    
    
   // multi compareing options useing if let operation & tuples:


    let first_name:Option<&str>=Some("eslam");
    let middle_name:Option<&str>=Some("mohamed");
    let last_name:Option<&str>=Some("elabd");

    if let (Some(first_name_value),Some(middle_name_value),Some(last_name_value))=(first_name,middle_name,last_name){
        println!("my name is : {first_name_value} {middle_name_value} {last_name_value}");
    }
    
    // unwarping with default value useing unwarp_or(default_value) , if an Option hasn,t no value it will be equal to default value

        
    println!("myname is {}",first_name.unwrap_or("eslam"));


    // unwarping with default function useing unwarp_or_else(callback function) 


    println!("my name is {}",none_name.unwrap_or_else(||->&str{println!("there is no value , we will assign to default value ");return "eslam";}));


   //unwarping with rust default_value for data type unwrap_or_default()

    let age :Option<u8>=None;
    println!("my age is {}",age.unwrap_or_default()); // age.unwrap_or_default() = 0 


    // is_some(&self)->bool or is_none(&self)->bool to check if a option has a value or not 

        if first_name.is_some(){
            println!("first_name has a value")
        }

        if none_name.is_none(){
            println!("last_name_value");
        }
    
        //mapping an option usein map(&self , function )->Option
        

        let age : Option<u8>= Some(10);
        println!("my age is {} ", age.map(|x|x*2).unwrap_or_default() );


}
