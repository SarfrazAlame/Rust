
// #[derive(Debug)]
// struct User {
//     username:String,
//     password:String,
//     age:u32
// }
 

// fn main(){
//     let u = User{
//         username:String::from("Sarfraz"),
//         password:String::from("Sarfraz"),
//         age:21
//     };

//     print!("{:?}", u); //debug
//     // print!("{}", u); // display
// }


//////////////////////////////////////////////////manual

use std::fmt::{write,Display};

struct User {
    username:String,
    age:u32,
}

impl Display for User {
    fn fmt(&self, f:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(f, "this is the user struct with age {}", self.age)
    }
}

fn main(){
    let u = User{
        username:String::from("Sarfraz"),
        age:21  
    };

    print!("{}", u.username);
    print!("{}", u.age);

    print!("{}", u)
}