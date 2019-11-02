//pointer = has the address to the value
//smart pointer = data structure like a pointer with  additional metadata
//aka box pointer to allocate data to heap, whereas without box, values will be allocated to the stack


enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn main() {
    let b = Box::new(18);
    println!("{}", b);//18

    //cons list = recursive call of cons operator until it creates a list
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));


    let y = 4;
    let x = &y;
    let z = Box::new(y);
    if *x == *z {
        println!("True")
    }

    ///closure
    let f = |i: i32| i + 1; //you can do it without annotation aka i32
    let x = 10;
    println!("{}", f(x));

    let p = || println!("null????");
    p();

    let mut c = 0;
    let mut inc = || {
        c += 1;
        println!("results {}", c);
    }; //inc closure function is borrowing "c" variable in this main scope

    inc();
    inc();
    inc();


    let p = || println!("hello from function p");
    run(p);

    let x1 = |i| (i * 10);
    println!("{}", add3(x1));

    let a1 = A {
        f: x1
    };
    let p1 = || println!("just printing");
//    run2(p1);
    run2(p2);

    let x = create();
    x();

    let v = vec![1,2,3];

    println!("{}", v.iter().any(|&x| x != 2));
}


fn create() -> Box<Fn()>{
    //move the closure outside after this scope has finsihed
    Box::new(move || println!("moved to here"))
}

fn p2() -> i32 {
    println!("from p2");
    8
}

/*
a function called "run" with a generic implementation and receives a parameter which also a generic of F
where this whole function with F as generic has to be a type of Fn
*/
fn run<F>(f: F) where F: Fn() {
    f();
}

fn add3<V>(f: V) -> i32 where V: Fn(i32) -> i32 {
    f(3)
}

struct A<F: Fn(i32) -> i32> {
    f: F
}

fn run2<F>(f: F) -> i32 where F: Fn()-> i32{
    let a = f() +8;
    println!("16? {}", a);
    a
}