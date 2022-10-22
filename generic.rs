struct SomeOne <T,U>{
    field1:T,
    field2:U
}

fn find_smallest_number<T>(array:&[T])->&T where T:std::cmp::PartialOrd{
    let mut smallest : &T =&array[0];
    for i in array{
        if smallest > i{
            smallest=i;
        }
    }
    return smallest;
}


fn main(){
let some_one:SomeOne<String,u8>=SomeOne{field1:String::from("eslam mohamed elabd"),field2:20};
let smallest=find_smallest_number(&[1,2,3,4,5,0]);

}
