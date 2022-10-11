fn array(){
    

    // array in rust is a collection of elements have same type with a fixed size
    // array : [type;size]

    let mut nums:[i8;5]=[1,2,3,4,5];
    let nums2:[i8;10]=[2;10];// this will generate an array with 10 elements which are every one  equal to  2
    
    //to access element of array we use the common way of accessing elements of an array by index.
    println!("value of second element in nums is {}",nums[1]); 
    
    //print : nums are [1,2,3,4,5]
    println!("nums are {:?}",nums);
    
    //changeing value of element in  a mutable array
    nums[1]=3;
    
    // array.len() to get size of array : len(&self)->usize
    println!("size of nums is {}",nums.len());
    
    //looping through an array
    // we have to ways to looping inside any thing
    

    //1) useing iterator method 
    

    for i in nums.iter(){
        println!("value of some element in nums : {}",i);
    }
    

    //2) with refference method 
    
    for &i in &nums{
        println!("value of some element in nums : {}",i);
    }

    
    //mapping an array  
    //[1,3,3,4,5]
   nums =nums.map(|x|{ x*4 }); // now nums equal [4,12,12,16,20]

}

