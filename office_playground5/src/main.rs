use std::ops;

trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64
}


impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.141 * self.radius * self.radius) as u32
    }
}

/*

    code for block comment #1
#[derive(Debug,Clone,Copy)] //Clone to clone the variable so you can assign to other variable
//Copy can be use so that you dont have to call .clone()
struct A(i32);

#[derive(Eq, PartialEq, PartialOrd, Ord)] //https://youtu.be/B9cHhfspDDE?t=242
struct B(i32);*/

struct A;

struct B;

#[derive(Debug)]
struct AB;

#[derive(Debug)]
struct BA;

impl ops::Add<B> for A {
    type Output = AB;
    fn add(self, _rhs: B) -> AB {
        AB
    }
}

impl ops::Add<A> for B {
    type Output = BA;

    fn add(self, _rhs: A) -> BA {
        BA
    }
}

struct C {
    c: string
}

impl Drop for C {
    //to drop the variable in memory at any given time. usage: in main() { ... drop(c) where c is the variable name}
    fn drop(&mut self) {
        println!("dropped {}", self.c)
    }
}

struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;
    //must have next() using Iterator implementation
    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;
        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib { c: 1, n: 1 }
}



fn main() {
    /*
       code for block comment #1
       let a = A(32);
       let c = a.clone();
       let c = a;*/
    println!("Hello, world!");
    println!("{:?}", A + B);
    println!("{:?}", B + A);

    //take the first 10 of the iterator
    for j in fib().take(10){
        println!("{}", j);
    }

    //skip first 14 values and take the first 10 afterwards
    for j in fib().skip(14).take(10){
        println!("{}", j);
    }

    let mut f = fib();
    println!("{:}", f.next());
}
