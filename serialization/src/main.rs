use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct User {
    username: String,
    password: String,
}

fn main() {
    let u = User {
        username: String::from("Sarfraz"),
        password: String::from("!23123"),
    };

    let serialized_string = serde_json::to_string(&u);

    // let user_string = serialized_string.unwrap();
    match serialized_string {
        Ok(str) => print!("{}", str),
        Err(_) => print!("Error while converting to string"),
    }
}

