use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}
impl Object {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let o = Object::new(50, 60);
    let k = Object::new(50, 50);

    o.show();
    k.show();
}
