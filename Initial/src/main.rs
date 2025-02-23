// fn main() {
//     // let ans: u32 = sum(2,3);
//     // println!("{}", ans)
//     // println!("Hello, world!");

//     // let ans:bool = is_even(7);
//     // println!("{}", ans)

//     // let name = String::from("Sarfraz");
//     // println!("{}", name)

//     // let v = vec![1,2,3];
//     // println!("{:?}", v)
// }

// fn sum(a:u32,b:u32)->u32{
//     return  a+b;
// }

// fn is_even(a:u32)->bool{
//     return a%2==0;
// }



// ////////////////////////////////////////
fn main(){
    let name = String::from("Sarfaraz");
    let len:usize =  get_len(name);
    println!("{}", len);

}

fn get_len(s:String)->usize{
    return s.len();
}

