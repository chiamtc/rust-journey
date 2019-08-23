fn main() {
    println!("Hello, world!");

    //1
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //2
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}",s2);

    //3
    let mut s3 = String::from("hello");

    {
        let r1 = &mut s3;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s3;
    println!("{}", r2);


    let r3 = &mut s3;
    println!("{}", r3);

    //5
    let s5 = no_dangle();
}
//1
fn calculate_length(s: &String) -> usize {
    s.len()
}

//2
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//5
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

