//Result Type Contain Ok() or Err() some like Option ,Some(),None
//Result<T,E> mean : Result<value_type,error_type>

fn error_handling(){


   //declareing an Result<valuetype,errtype>

    let mut some_result:Result<&str,&str>=Ok("value");
     
    // warping the result with matching 

    match some_result {
        Ok(value)=>{println!("value : {}",value)},
        Err(error)=>{println!("error happened : {}",error)}
    }
    

     some_result=Err("runtime error");
    
     match some_result {
        Ok(value)=>{println!("value : {}",value)},
        Err(error)=>{println!("error happened : {}",error)}
    }
    

    // expect(&self,msg:&str) warping result and if Error , then Exit or panic  with a runtime error in thread  with error type and error message


     some_result.expect("expecting no error");
    

    // expect_err (&self,msg:&str) warping result and if Ok and not  Err , then Exit or panic with a runtime error in thread  with expect message

    some_result.expect_err("expecting error");
    
    // is_ok(&self)->bool return true if  result return Ok(value);

    if some_result.is_ok(){
        println!("it,s ok ");
    }

    // is_err(&self)->bool return true if  result return Err(error);

    if some_result.is_err(){
        println!("there is an  error");
    }

    // mapping ok value , .map(&self,fn)->Result<fn return type , error type>

    let some_result_length=some_result.map(|s|s.len()).unwrap_or_default();

    // mapping Err error , .map_err(&self,fn)->Result<value type , fn return type>

    let error_length=some_result.map_err(|s|s.len()).unwrap_or_default();

    //panic!("error message") , panic is a macro that throw an error message and exit
    

    match some_result {
        Ok(value)=>{},
        Err(error)=>{panic!(" error : {} ", error)}
    }



}



fn main(){


}
