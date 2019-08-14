fn main() {
    /*
    Statements are instructions that perform some action and do not return a value. .e.g let x = 6;
    Expressions evaluate to a resulting value.  e.g. x+6 , calling a function, macros and use of {}
    */
    let x = 5;
    let z = five();
    let a: i32 = plus_one(z);
    let y = {
        let x = 3;
        x + 1 // <<<-- doesn't have semicolon (;) because it is an expression, otherwise, it is a statement, which will not return a value;
        //https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions
    };

    println!("{}, {}", z,a);
    another_function();
}

fn another_function() {
    println!("Another function");
}

fn five() -> i32 { //returns value of 5 -without semicolon https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions
    5
}

fn plus_one(val: i32) -> i32 {
    val + 1
}
