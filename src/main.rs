use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let array = [-1, 5, 8, -9, 4, 1];

    let aa = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    //println!("{}", trap(aa));
}

//try_into().unwrap() is a method chain in Rust that is used to convert one type of value into another.
//try_into() is a method that is defined on the std::convert trait, and it is used to attempt to convert a value of one type into a value of another type. It returns a Result object, which is an enumeration that can either be Ok and contain the converted value or Err and contain an error message.
//The unwrap() method is defined on the Result object and it is used to extract the value that is contained inside the Ok variant of the Result enum. If the Result object is an Err, unwrap() will panic and the program will exit.
//In other words, try_into().unwrap() is used to convert a value from one type to another and it will panic in case of error. It's a quick and convenient way to convert values, but it can be dangerous if the value is not guaranteed to be valid.
//It is recommended to use the expect() or ok_or() method instead of unwrap() when working with Result in production code, since they give more information about the error when it occurs.
