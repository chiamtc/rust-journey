use std::mem;
fn main() {
    //i8, u8, i16, u16, i32, u32, i64, u64;
    let a = 1 + 20;
    let s = 20 - 20;
    let m = 20 * 2;
    let d = 20 / 3;
    let r = 20 % 2;
    let c = 'c';
    let c: char = 'z';
    let x = 5;
    let mut y = 5;
    y = 10;

    let t: (i32, f64, char) = (1, 1.2, 's');
    //destructuring
    let (z, y, x) = t;
    let (_, _, x) = t; //x = 's', _ to skip value
    println!("{}", t.2); // 's'


    //array
    let arr = [1, 2, 3];
    let a1 = arr[0];

    println!("Hello, world!, {}", y);

    let t = (1, 'a', false);
    let f = (2, t);
    println!("{} {:?}", t.0, f.1);
    println!("{} {:#?}", t.0, f.1);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}, {}, {}", xs[0], xs.len(), mem::size_of_val(&xs)); //size in memory

    let ys = &xs[2..4];
    println!("{:?}", ys);

    let s = "String";
    let s1 = String::from("String");
    let slice = &s1[0..4];
    println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let ss = h + &w; // -> (self, &reference)
    println!("{}",ss)


}
