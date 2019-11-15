#[macro_use]
extern crate stdweb;

fn main() {
    let message="asdas";
    js!{
        console.log(@{message});
    }
}
