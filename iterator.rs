// iterator is a trait in rust

use std::iter::FromIterator;

fn main(){



    //creating an array 
    let array:[u8;6]=[1,2,3,4,5,6];
    println!("array content : {:?}",array);
    
    /*
     * for i in &array{
     *  
     * }
     * 
     * for i in array.iter(){
     *
     * }
     *
     * the 2 loops are the same.
     *
     * */ 

    //iter() & iter_mut()  will iterate over (im)mutable references 
    

    //sum() functional programming function will sum all elements 
    let sum:u8=array.iter().sum();
    println!("the sum of all array element is : {}",sum);


    //map() functional programming function  will evaluate an expersion for every element and return new Map with new elements 
    //the Map returned by map() need to be collected useing collect() to collect it into vectors or any thing stored in the heap

    let array_multiby_2:Vec<u8>=array.iter().map(|num|num*2).collect();
    println!("array_multiby_2 : {:?}",array_multiby_2);

    //filter() functional programming function will evaluate an case for every element and return new Filter with elements accept the case 
    //the Filter which returned by filter() need to be copied as iterator useing copied() 
    //then collected useing collect() to collect it into vectors or any thing stored in the heap

    
    let array_less_than_3:Vec<u8>=array.iter().copied().filter(|num|num<&&3).collect(); 
    println!("array_less_than_3 : {:?}",array_less_than_3);


    //into_iter() will take a copy of the array and loop inside it not into references and have the same functionality like iter()

    /*
     * for i in array{
     *
     * }
     *
     * for i in arrat.into_iter{
     *
     * }
     *
     * the 2 loops are the same
     *
     * */


}
