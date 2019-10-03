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
}
