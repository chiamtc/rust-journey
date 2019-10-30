use std::fmt;
struct Square<T>{
    x:T
}

fn  p<T:fmt::Debug> (x:T){
    println!("{:?}",x)
}

struct A<T>{
    x:T
}

//T here is to give implementation A a scope
//we can change T to any name we want as long as the entire impl block has the same name
//E.g.
/*
impl <C> A<C>{
    fn item(&self) -> &C{
        &self.x
    }
}
*/
impl <T> A<T>{
    fn item(&self) -> &T{
        &self.x
    }
}

fn main() {
    let s = Square{x:10};
    let s = Square{x:1.0};
    let s = Square{x:"Hello"};
    let s = Square{x:'c'};

    p(10);

    let a = A{x:"Hello"};
    a.item();

}
