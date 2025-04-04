use std::{fmt::format, path::Display};

#[derive(Debug)]
struct User {
    username: String,
    password:  String,
    age: u32
}
 

fn main(){
    let user =  User{
        username:String::from("Sarfraz Alam"),
        password:String::from("123456"),
        age:20
    };

    println!("{:?}", user); // debug
}