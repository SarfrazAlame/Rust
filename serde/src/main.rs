// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct User {
//     username: String,
//     password: String
// }

// fn main() {
//     let person = User{
//         username:String::from("sarfraz"),
//         password:String::from("alsjf")
//     };

//     // let json_str = serde_json::to_string(&person); 
//     let json_str = serde_json::to_string(&person).unwrap();

//     println!("{}", json_str)

//     // match json_str {
//     //     Ok(str)=> println!("{}", str),
//     //     Err(_)=> println!("error while converting")
//     // }
// }


/////////////////////////
/// 
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String
}


fn main(){
    let s = String::from("{\"username\":\"sarfraz\", \"password\":\"12345\"}");
    let u:Result<User, serde_json::Error> = serde_json::from_str(&s);

    println!("{:?}", u.unwrap())
}

