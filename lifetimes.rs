/*lifetimes come to solve common problem 
for example :


fn return_ref_str()->&str{
return "hello";
}

fn main(){
println!("{}",return_ref_str);
}
 thats will compile with error because the &str which returned from return_ref_str deallocated after finish executing function return_ref_str so we need
 to specify a life time to tell the compiler that this refference will be life after function executed 
*/


// 'static life time mean that refference will lives over the program  untill it finish executation


fn static_lifetime_str()->&'static str{
return "hello iam string slice";
}

// another example

fn static_login_status(user:&'static str,pass:&'static str)->&'static str{
    if user=="eslam" && pass=="eslam" {
        return  "login success";
    }
    else{
        return  "login failed !";
    }
}


// define generic lifetimes

fn generic_lifeime_ref_u8<'a>()->&'a u8{
return &4;
}

//another example 

fn generic_login_status<'l1,'l2>(user:&'l1 str,pass:&'l2 str)->&'l1 str{ // <l1,l2> here we told function that we have  2 lifetimes l1,l2 
                                                                         // and return a &str live as user live 
                                                                         // we can define & use <l1> for each eigther 
   if user=="eslam" && pass=="eslam" {
        return  "login success";
    }
    else{
        return  "login failed !";
    }

}

//useing lifetimes with structs

struct Person{
name:&'static str, //thats mean name will be live over program executation
age:u8
}

struct User<'l1>{
username:&'l1 str, //thats mean username & pass will live as the instance of User live
pass :&'l1 str 
}


// ## The 3 Lifetimes rules 

/*
 * 1) - if your function has parameters and parameters one or more of them is a refference  and return type is the a refference  and the same type 
 * compiler add automaticly lifetimes to function and the return type  while compile
 */

// Example on lifetimes_rule1:

fn lifetimes_rule1<'a>(age:u8,name:&str)->&str{return "new name";} 
// compiler automaticly add lifetimes -> fn lifetimes_rule1<'a>(age:u8,name:&str)->&'a str{return "new name";}


/*
 * 2) - if your function has parameters and parameters one or more of them is a refference  and return type  is the a refference  and the same type 
 *  and return is one of the parameter[s]  which  is same type also 
 *  compiler add automaticly lifetimes to function and  the parameter and the return type   while compile
 */

fn lifetimes_rule2<'a>(age:u8,name:&str)->&str{return name;} 
// compiler automaticly add lifetimes -> fn lifetimes_rule2<'a>(age:u8,name:&'a str)->&'a str{return name;}


/*
 * 3)- if one of your parameters is a &self or &mut self so the life time of self will be the same lifetimes of return type 
 *
 */

struct LifetimesRule3<'l1>{
some_field:&'l1 str
}

// small note : impl<'lifetime> mean we define this lifetime to use with the all implementation code 
//it like fn<'lifetime> 

impl <'l1> LifetimesRule3 <'l1>{
    
// now the returned reffence lifetime will be the same lifetime of self mean as self live the returned refference will live
    fn get_some_field(&self)->&str{
        return &self.some_field;
    }

    fn get_lastest_char(&self)->&str{
        return &self.some_field[self.some_field.len()-1 .. self.some_field.len()];
    }
}


// and everything we apply with structs will be the same as enums 

enum Human<'l>{
Male{name:&'l str},
Female{name:& 'l str}
}
impl  <'l> Human <'l> {
    fn say_gender_and_name(&self)->&str{
        match self {
            Self::Male { name }=>{println!("name is {} & gender is male ",name);return *name;},
            Self::Female { name }=>{println!("name is {} & gender is female",name);return *name;}
        }
    }
}



fn main(){
let rule3:LifetimesRule3=LifetimesRule3 { some_field: "eslam"};
println!("{}",rule3.get_lastest_char());
}
