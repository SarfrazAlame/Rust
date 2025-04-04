use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

fn main() {
    let person = User{
        username:String::from("sarfraz"),
        password:String::from("alsjf")
    };

    let json_str = serde_json::to_string(&person).unwrap();
    println!("Serialize JSON: {}", json_str);
}
