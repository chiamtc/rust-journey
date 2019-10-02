/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main(){
    let rect1 = Rectangle { width:30 , height:32};
    println!("rect1 area is: {:?} ", rect1);
}
