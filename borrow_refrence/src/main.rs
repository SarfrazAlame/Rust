// fn main() {
//     let str = String::from("Sarfraz");

//     let str1 = &str;
//     let str2 = &str;

//     println!("{} {}", str1, str2)
// }

// fn main(){
//     let str = String::from("Sarfraz");
//     let len = get_len(&str);

//     println!("{} {}", len, str);
// }

// fn get_len(s:&String)->usize{
//     return s.len();
// }


// fn main() {
//     let mut str = String::from("Harkirat");
//     let ref1 = &mut str;
//     ref1.push_str("Singh");
//     let ref2 = &str;
    
//     println!("{}", ref2);
// }


struct Rect{
    height:f32,
    width:f32
}

fn main(){
    let r = Rect {
        height:10.0,
        width:10.0
    }

    println!("{} {}", r.height, r.width);

}