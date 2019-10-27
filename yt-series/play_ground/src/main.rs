fn main() {
    //i8, u8, i16, u16, i32, u32, i64, u64;
    let a = 1+20;
    let s = 20-20;
    let m = 20*2;
    let d = 20/3;
    let r = 20%2;
    let c = 'c';
    let c:char ='z';
    let x = 5;
    let mut y = 5;
    y = 10;

    let t: (i32, f64, char) = (1, 1.2, 's');
    //destructuring
    let (z,y,z)= t;
    let (_, _, x) = t; //x = 's', _ to skip value
    println!("{}",t.2); // 's'


    //array
    let arr = [1,2,3];
    let a1 = arr[0];

    println!("Hello, world!, {}", y);
}
