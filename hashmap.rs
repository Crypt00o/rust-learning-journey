use std::collections::HashMap;
use std::collections::hash_map::Entry;


fn print_value(my_hashmap:&HashMap<&str,&str>,key:&str){

    if my_hashmap.contains_key(key){
        println!("<{}:{}>",key,my_hashmap[key]);

    }
}

fn create_if_not_exists<'a>(hashmap:&mut HashMap<&'a str,& 'a str>,key:&'a str,value:& 'a str)->bool{
    if !hashmap.contains_key(key){
        hashmap.insert(key,value);
        println!("creating {}:{}",key,value);
        return  true;
    }
    else{
        return false;
    }
}

fn update_if_exists<'a>( hashmap:&mut HashMap<&str,&'a str>,key:&str,value:&'a str)->bool{

    if hashmap.contains_key(key){
        println!("updateing {}",key);
        match  hashmap.get_mut(key){
        Some(_value)=>{*_value=value;return true},
        None=>{return false;}
        }
    }
    else {
        return false;
    }

}

fn delete_if_exists<'a>(hashmap:&mut HashMap<&'a str ,&'a str> , key: &'a str )->bool{
    if hashmap.contains_key(key) {
        hashmap.remove(key);
        return  true;
    }
    else{
        return false;
    }

}




fn main(){
    let mut my_hashmap :HashMap<&str,&str>=HashMap::new();
        // create key if not exists 

    let mut key:&str="name";
        if create_if_not_exists(&mut my_hashmap, key, "eslam mohamed"){
            print_value(&my_hashmap, key);
        }
        if update_if_exists(&mut my_hashmap , key, "eslam"){
            print_value(&my_hashmap, key);
        }
        if delete_if_exists(&mut my_hashmap, key){
            print_value(&my_hashmap, key);
        }



        // create on the go

        my_hashmap.entry("job").or_insert("exploit devloper");
        
        // unsafe access key if not found it will show an panic
        
        println!("access job value with unsafe way {}",my_hashmap["job"]);
        
        // access value with safe way
        
        match my_hashmap.get("job")  {
            Some(_value)=>{println!("now job access with the safe way 'matching Options which return by hashmap.get(key)' : {} ",_value)},
            None=>{ println!("sorry job key not exists")}
        }
        
        // access key with another safe way "with entry"
        
        match my_hashmap.entry("job")  {
            Entry::Occupied(_value)=>{ println!("now job access with the safe way 'matching entry' : {} ", _value.get())},
            _=>{ println!("sorry job key not exists")}
         }
        




        // to use structs or enums keys we should use macro derive() with traits Hash,Eq,PartialEq

        #[derive(Hash,PartialEq,Eq)]
        enum Gender{
            MALE,FEMALE
        }


        #[derive(Hash,PartialEq, Eq)]
        struct Person{
            name:String,
            age:u8,
            gender:Gender
        }
        
        let  struct_hashmap_example:HashMap<Person,&str>=HashMap::new(); 

}
