use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} = area {}", self.width, self.height, self.area())
    }
}

impl fmt::Display for Object{
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
       write!(f, "({}, {}) ", self.width, self.height)
   }
}

impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

fn main() {
    println!("Hello, world!");

    let o = Object::new(2,1);
    o.show();

    let obj = Object::new(2,2);
    obj.show();
    println!("{}", obj)
}
