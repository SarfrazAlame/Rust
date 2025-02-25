use chrono::{Utc, Local};

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();

    println!("{}", local_time);
    print!("{}", utc)
}

/////////////////////////////////////////////////////////////////////////
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("REDIS_ADDRESS");

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => print!("Error while reading variable"),
    }
}

///////////////////////////////////////////////////////////////////  generics traits////////////////

fn main(){
    print!("{}", sum(1, 2))
}

fn sum_num<T>(a:T,b:T)->T{
    return a+b;
}

fn sum<T: std::ops::Add<Output=T>>(a:T,b:T)->T{
    return a+b;
}

struct User {
    username:String
}

fn main(){
    print_variable(1);
    print_variable(3.2);
    print_variable(String::from("Sarfraz"));
}

fn print_variable<T:std::fmt::Display>(a:T){
    print!("{}", a)
}

///////////////////////////////////////////////////////generics over struct


struct Rect<T>{
    width:T,
    height:T
}

impl<T:std::ops::Mul<Output = T> +Copy> Rect<T> {
    fn area(&self)->T{
        return  self.height * self.width;
    }
}

fn main(){
    let r = Rect{
        width:10,
        height:10
    };

    let r1 = Rect{
        width:10.0,
        height:10.0
    };

    print!("{}", r.area());
    print!("{}", r1.area())
}

/////////////////////////////////////////////////////// traits
use std::f32::consts::PI;

trait Shape {
    fn area(&self)->f32;
}
struct Rect{
    width:f32,
    height:f32
}

impl Shape for Rect {
    fn area(&self)->f32{
        return  self.width*self.height
    }
}

struct Circle{
    raduis:f32
}

impl Shape for Circle {
    fn area(&self)->f32{
        return PI * self.raduis*self.raduis;
    }
}


fn print_area_of_shape<T:Shape>(s:T){
    print!("{}", s.area())
}

fn main(){
    let r = Rect{
        width:10.0,
        height:10.0
    };

    let c = Circle{
        raduis:10.0
    };

    print_area_of_shape(r);
}
