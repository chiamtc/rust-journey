#![allow(dead_code)] //ignore warning code

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}


impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            //ref s = scenario below
            /*
                let u = 10;
                let v = &u;
                let ref z = u;
                z === u; true
            */
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r)
        }
    }
}


fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let u = Direction::Up(Point { x: 0, y: 1 });
    let k = u.match_direction();
    println!("{:?}", k);
    let x = k.destruct();
    println!("{}", x);

    let rect = Shape::Rectangle { width: 2, height: 3 };
    println!("{}", rect.area());

    let circle = Shape::Circle(4.5);
    println!("{}", circle.area());

    let res = division(2.0,0.0);
    let matcher = match res {
        Some(x) => x,
        None => 0.0
    };
    println!("{}", matcher)
}
