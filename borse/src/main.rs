use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize, Debug, Clone)];

struct User {
    username: String,
    password: String
}

fn main() {
    let u = User {
        username: String::from("Sarfraz"),
        password: String::from("123213")
    };

    let mut v: Vec<u8> = Vec::new();

    println!("{:?}", v)

    let ans = u.serialize(&mut v)

    let user = User::try_from_slice(&v).unwrap()

    print!("{}", user.username)
}
