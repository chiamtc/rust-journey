mod A;
mod C;

use A::B;
#[derive(Debug)]
enum Ex{
   D,E,F
}

use Ex::{D,E};

fn main(){
    A::b();
    B::a();

    println!("{:?}", D)
}