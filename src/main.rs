extern crate piston_window;
extern crate rand;

mod draw;

fn main() {
    println!("Hello World");
}
/*

use std::fs::File;
fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}
 */

/*
fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.0, 7.0);
    match res {
        Some(x) => println!("{:.10}", x),
        None => println!("cannot divide by 0"),
    }
}
 */

/*
enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r) as f64, //vec je f64
        }
    }
}

fn main() {
    let r = Shape::Rectangle {
        width: 10,
        height: 70,
    };
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);

    let ar = r.area();
    println!("{}", ar);

    let ass = s.area();
    println!("{}", ass);

    let ac = c.area();
    println!("{}", ac);
}
*/

/*
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

    println!("{}", o);
}

 */
