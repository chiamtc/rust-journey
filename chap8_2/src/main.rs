fn main() {
    let mut s = String::new();
    let data= "initial_contents";
    s = "initial_contents".to_string(); //or data.to_string()
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    s.push_str(" bar");
    println!("{}",s);
    println!("Hello, world!");


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); //doesn't take the ownership
    println!("s2 is {}", s2);

    let s4:String = String::from("hello, ");
    let s5:String = String::from("world");
    let s6:String = s4 + &s5; // + operator implementation beneath is `fn add(self, s: &str) -> String {}`
    // note s4 has been moved here and can no longer be used because of defer coercion
    println!("{}", s6);


    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    //format!
    let s = format!("{}-{}-{}", s7, s8,s9);


    let len = String::from("Hola").len();
    let t = "abc";
    let indexing = &t[0..1];
    println!("{}, {}", len, indexing);
}
