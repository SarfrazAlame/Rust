
trait Shape {
    fn area(&self) ->u32;
}

struct Rect {
    width:u32,
    height:u32
}

struct Circle {
    radius:u32
}

impl Shape for Rect {
    fn area(&self)->u32{
        return self.width * self.height;
    }
}

impl Shape for Circle {
    fn area(&self)->u32{
        return self.radius*self.radius;
    }
}

fn main() {
    let r = Rect {
        width:10,
        height:10
    };
    let c = Circle {
        radius:7
    };

    let ar_ci = get_area(c);
    println!("{}", ar_ci);

    let ar = get_area(r);
    println!("{}", ar);
}

fn get_area(s:impl Shape)->u32{
    return s.area();
}

