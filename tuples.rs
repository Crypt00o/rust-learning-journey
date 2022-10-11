 fn tuples(){
    //tuples
    let me:(&str,&str,u8)=("eslam","Software Engineer",20);
    let mut me2=me;
    
    // to access tuples , we use index but in with the way we access structs 
    // tuples are structs on the go

    me2.0="eslam mohamed";

    // tuples structs
    
    struct Cube(f32,f32,f32);
    let cube:Cube=Cube(11.0,12.0,10.0);

 }
