fn main() {
    let x; //think of 'a
    {
        let y = 10; //think of 'b
        x = &y;
    }

    println!("Hello, world!");

    let a = A{x:"hello"};

    //static lfietime
    let s:&'static str = "The string";
}

//if we return first parameter of this func, we dont have to do lifetime
// if we create a new variable in the function, then we have a dangling reference. To solve it, a lifetime annoataion is requried
fn pr<'a>(x:&'a str, y:&'a str) -> &'a str{
    if x.len() == y.len(){
        x
    }else{
        y
    }
}



//if struct holds a reference, it needs a lifetime annotation
//every reference behind the scene of rust has lifetime annotation aka lifetime illusion/inference
// but it doesn't last long enough
struct  A<'a>{
    x:&'a str
}