struct Point<T>{
    x:T,
    y:T
}

/*
Note that we have to declare T just after impl so we can use it to specify that we’re implementing methods on the type Point<T>.
By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
*/
impl<T> Point <T>{
    fn x(&self) -> &T{
        &self.x
    }
}

fn main() {
    let p = Point{x:5, y:10};

    println!("p.x= {}", p.x());
}
/*

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition.
Here, the generic parameters T and U are declared after impl, because they go with the struct definition. The generic parameters V and W are declared after fn mixup, because they’re only relevant to the method.

Rust uses - Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
*/
