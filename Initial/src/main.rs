// fn main(){
//     let ans: u32 = sum(1,3);
//     println!("{}", ans)
// }

// fn sum(a:u32,b:u32)->u32{
//     return a+b;
// // }

// fn main() {
//     let ans = is_even(3);
//     println!("{}", ans)
// }

// fn is_even(a: u32) -> bool {
//     return a % 2 == 0;
// }

// fn main(){
//     for i in 0..100{
//         println!("{}", i)
//     }
// }


// fn main(){
//     let name = String::from("sarfraz");
//     let (len, name) = get_len(name);

//     println!("{}", len);
//     println!("{}", name);
// }

// fn get_len(s:String)->(usize, String){
//     return (s.len(), s);
// }

// fn main() {
// let ans: u32 = sum(2,3);
// println!("{}", ans)
// println!("Hello, world!");

// let ans:bool = is_even(7);
// println!("{}", ans)

// let name = String::from("Sarfraz");
// println!("{}", name)

// let v = vec![1,2,3];
// println!("{:?}", v)
// }

// fn sum(a:u32,b:u32)->u32{
//     return  a+b;
// }

// fn is_even(a:u32)->bool{
//     return a%2==0;
// }

// ////////////////////////////////////////////////////// String
// fn main(){
//     let name = String::from("Sarfaraz");
//     let len:usize =  get_len(name);
//     println!("{}", len);
// }

// fn get_len(s:String)->usize{
//     return s.len();
// }

// ////////////////////////////////////////////////////// borrowing
// fn main() {
//     let s1 = String::from("Sarfrazz");
//     let len = get_len(&s1);

//     print!("{}", len);

//     print!("{}", &s1);
// }

// fn get_len(s2: &String) -> usize {
//     return s2.len();
// }

// fn main() {
//     let str = String::from("Sarfraz");

//     let ref1 = &str;
//     let ref2 = &str;
//     let ref3 = &str;

//     println!("{} {} {}", ref1, ref2, ref3)
// }

// ////////////////////////////////////////////////////// struct

// struct Rect {
//     height:f32,
//     width:f32
// }

// fn main(){
//     let r = Rect{
//         width:10.0,
//         height:10.0
//     };

//     println!("{} {}", r.width, r.height)
// }

// struct Rect {
//     height: f32,
//     width: f32,
// }

// impl Rect {
//     fn area(&self)->f32{
//         return self.height * self.width;
//     }

//     fn perimeter(&self)->f32{
//         return  2.0 * (self.height + self.width);
//     }

//     fn print_something(a:u32){
//         println!("Static function")
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 10.0,
//         height: 10.0,
//     };

//     println!("{} {}", r.width, r.height);
//     print!("{}", r.area());
//     Rect::print_something(10);
// }

// //////////////////////////////////////////////////////////enum
// enum Direction {
//     North,
//     South,
//     West,
//     East,
// }

// fn main() {
//     let directoin = Direction::North;

//     steer(directoin)
// }

// fn steer(dir: Direction) {
//     match dir {
//         Direction::East => print!("East Direction"),
//         Direction::West => print!("West Direction"),
//         _ => println!("Horizontal direction")
//     }
// }
// use std::f32::consts::PI;

// enum Shape {
//     Sqare(f32),
//     Cirlce(f32),
//     Rectangle(f32, f32),
// }

// fn main() {
//     let shape = Shape::Sqare(10.0);
//     let shape_Circle = Shape::Cirlce(10.0);
//     let shape_rect = Shape::Rectangle(10.0, 10.0);

//     print!("{}", calculate_area(shape_Circle))
// }

// fn calculate_area(s: Shape) -> f32 {
//     match s {
//         Shape::Cirlce(radius) => PI * radius * radius,
//         Shape::Sqare(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

// //////////////////////////////////////////////////////////////////////////////////// reading file

// use std::fs;

// fn main(){
//     let contents = fs::read_to_string("a.txt");

//     match contents {
//         Ok(contents)=>println!("{}", contents),
//         Err(e)=>println!("Error while reading file")
//     }
// }

// enum Option1 {
//     Some(u32),
//     None,
// }

// fn main() {
//     let ans = find_first_a(String::from("dlkglfglkdj"));

//     match ans {
//        None => print!("value not found"),
//        Some(val) => print!("a found at index {}", val),
//     }
// }

// fn find_first_a(str: String) -> Option<u32> {
//     let mut index = 0;
//     for c in str.chars() {
//         if c == 'a' {
//             return Some(index);
//         }
//         index = index + 1;
//     }
//     None
// }
