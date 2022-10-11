fn vectors(){
    // Vectors is a contiguous resizable array type, with heap-allocated contents . like an array but they are resizable .
   
    // ways to delare a vector :
   
    //useing Vec::from(array)
    let my_victor:Vec<i32>=Vec::from([1,2,3,4,5]);
   
    // useing Vec::new();
    let my_victor:Vec<i32>=Vec::new();
    
    //useing macro vec!
    let mut my_victor:Vec<i32>=vec![1,2,3,4,5];
    

    //push(&mut self) method in vectors add new element after the last element in the vector
    
    my_victor.push(1000);

    for i in my_victor.iter(){
        println!("value of some element in my_victor : {}",i);
    }
    
    println!("now length of my_vector is : {}",my_victor.len());
    

    // pop(&mut self)->T methods in vectors remove last element from vector and return it as (Optional Type : "to get the raw value we use unwarp() method " ) 
    println!("poping from my_vector..."); 
    my_victor.pop();

    println!("now length of my_vector is : {}",my_victor.len());
    

    println!("my_vector contain {:?} ",my_victor);
    
    let mut another_vector:Vec<i32>=Vec::from([97,98,99,100]);
    

    //clone  and append vector with extend_from_slice(&mut self,&Vec<T>) method ; 
    println!("extending my_vector...");
    my_victor.extend_from_slice(&another_vector); //now my_vector contain [1,2,3,4,5,97,98,99,100]
    println!("now my_vector contain {:?}",my_victor);
    
    //move another vector value to this vector useing append(&mut self,&mut Vec<T>) method 
    println!("now another_vector contain {:?}",another_vector);
    my_victor.append(&mut another_vector);
    println!("now my_vector contain {:?}",my_victor);  //now my_vector contain [1,2,3,4,5,97,98,99,100,97,98,99,100]
    println!("now another_vector contain {:?}",another_vector); // now another_vector contain [] 
    

    //contains(&self,&T)->bool vector method to check if vector contains a value or not
    println!("is my_victor contain 5 ? = {}",my_victor.contains(&2));

    println!("clearing from my_vector..."); 

    //clear(&mut self) vector method clear a vector and make it empty
    my_victor.clear();
    println!("now length of my_vector is : {}",my_victor.len());

    //is_empty(&self)->bool vector method to check if vector is empty or not 
    println!("is my_victor is empty ? = {}",my_victor.is_empty());

}

