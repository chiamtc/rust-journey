fn main() {
    //mut = mutable
    //without mut = immutable
    //in compile time, it throws error if we re-assign value to immutable variable

    //u32 = unsigned = cannot hold negative number
    //i32 = unsigned = can hold negative number
    //https://medium.com/@marcinbaraniecki/on-integer-types-in-rust-b3dc1b0a23d3
    let y: i32 = -5;
    let x: i32 = 5;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
    //shadowing

    /*
    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
    if we accidentally try to reassign to this variable without using the let keyword. By using let,
     we can perform a few transformations on a value but have the variable be immutable after those
     transformations have been completed.

    The other difference between mut and shadowing is that because we’re effectively creating a new variable
    when we use the let keyword again, we can change the type of the value but reuse the same name. For example
    , say our program asks a user to show how many spaces they want between some text by inputting space
    characters, but we really want to store that input as a number:
    */
    let spaces = "    ";
    let spaces = spaces.len();
}
