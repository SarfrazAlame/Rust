// fn main() {
// //    let arr  = [1,2,3,4,5];
// //    println!("{}", arr.len())

//     // let mut xs = vec![1,2,3,4];
//     // print!("{}", xs.len());

//     // print!("Hl");

//     // xs.push(4);
//     // print!("{}", xs.len());
// }

// conditionals , loops
// pub fn main(){
//     let x = 12;
//     let num = is_even(x);

//     if num {
//         println!("{} is even", x);
//     }else {
//         println!("{} is odd", x);
//     }
// }

// pub fn is_even(x:i32)->bool {
//     return  x%2 == 0;
// }

// loops first name

// pub fn main(){
//     let str = String::from("Sarfraz Alam");
//     println!("{} is the character of your name", str)
// }

// pub fn get_first_name(x:String)->String{
//     let mut name = String::from("");

//     for c in x.chars() {
//         if c == ' ' {
//             break;
//         }
//         name.push(c);
//     }

//     return name;
// }

/////////////////////////////// Ownership in rust
///

// fn main() {
//     let name = String::from("Sarfraz Alam");

//     let (name, len) = get_len(name);

//     println!("{}", len);
//     println!("{}", name)
// }

// fn get_len(x: String) -> (String,usize) {
//     let len = x.len();
//     return (x, len)
// }


///////////////////////////////////////////////////// struct in Rust

// #[derive(Clone, Copy)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count:u64,
//     age:u32
// }

// fn main(){
//     let mut user1 = User {
//         active:true,
//         username: String::from("Sarfraz Alam"),
//         email:String::from("sarfraz@123gmail.com"),
//         sign_in_count:1,
//         age:22
//     };

//     print_name(user1);

//     print!("User 1 username : {}", user1.username)
// }


// fn print_name(user1: User) {
//     print!("User 1 username: {}", user1.active);
// }

///////////////////////////////////////////////////// implementing struct in Rust
// struct Rect{
//     width:u32,
//     height:u32,
// }

// impl Rect {
//     fn area(&self)->u32{
//         self.width * self.height
//     }
// }

// impl Rect {
//     fn perimeter(&self)->u32{
//         self.width+self.height
//     }
// }

// fn main(){
//     let rect = Rect {
//         width:30,
//         height:50,
//     };


//     println!("the area of the rectangle is {}", rect.area());
//     println!("the perimeter of the rectangle is {}", rect.perimeter());
// }


// struct Rect {
//     width: u32,
//     height: u32,
//  }
 
//  impl Rect {
//      fn print_str() {
//         println!("Inside the rect struct");
//      }
//  }
 
//  fn main() {
//     Rect::print_str();
//  }


///////////////////////////////////////////////////////////////////////////////////  Pattern matching

// enum Shape {
//     Circle(f64),
//     Sqaure(f64),
//     Rectangle(f64, f64),
// }

// fn calculate_area(shape: Shape)->f64{
//    match shape {
//        Shape::Circle(radius)=> std::f64::consts::PI * radius * radius,
//        Shape::Rectangle(width, height )=> width*height,
//        Shape::Sqaure(side_length)=>side_length*side_length,
//    }
// }

// fn main(){
//     let circle = Shape::Circle(5.0);
//     let sqare = Shape::Sqaure(4.0);
//     let ractangle = Shape::Rectangle(4.0,2.0);

//     println!("Area of circle : {}", calculate_area(circle));
//     println!("Area of ractangle : {}", calculate_area(ractangle));
//     println!("Area of sqare : {}", calculate_area(sqare));
// }


///////////////////////////////////////////////////////////////  traits in rust

// trait Shape {
//     fn area(&self) -> f32;
// }

// struct Rect {
//     width: f32,
//     height: f32
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height
// 	  }
// }

use std::fmt;
struct Rect {
	width: u32,
	height: u32
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let s = Rect {
        width: 100,
        height: 100
    };
    println!("{}", s);    
}

