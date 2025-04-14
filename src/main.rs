use std::collections::HashMap;
use std::io;




fn main(){
    //generics and traits
   
    let char_container:Vec<char> = "abcdefghijklmnop".chars().collect();
    let num_container:Vec<i32> = (1..20).collect();

    let app =generic_largest(&char_container);
    let app2 = generic_largest(&num_container);



}


fn generic_largest<T:std::fmt::Display+std::cmp::PartialOrd>(param:&[T]) ->&T{
    let mut largest = &param[0];
    for item in param{
        if item > largest{
            println!("{item} is larger than {largest}");
            largest = item;
        }
    }
    largest
}