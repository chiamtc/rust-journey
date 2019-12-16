fn main(){
    print_a(&vec!["hello".to_string(), "world".to_string()]);
    print_b(&["hello","world"]);

    print_any(&["hello".to_string(),"world".to_string()]);
    print_any(vec!["hello","world"]);
}

fn print_a(v:&Vec<String>){
    println!("a");
    for(i, val) in v.into_iter().enumerate(){
        println!("{} == {}" , i , val);
    }
}

fn print_b(v:&[&str]){
    println!("b");
    for(i,val) in v.into_iter().enumerate(){
        println!("{} == {}" , i,val);
    }
}

fn print_c<I:Iterator<Item=String>>(v:I){
    println!("c");
    for(i,val) in v.into_iter().enumerate(){
        println!(" {} == {} ", i, val);
    }
}
//as reference of &str or String. if String then it returns to a pointer of str . if it's &str then it will return &str
fn print_any<S:AsRef<str>, I:IntoIterator<Item=S>>(v:I){
    for(i,val) in v.into_iter().enumerate(){
        println!("{} == {}", i, val.as_ref());
    }
}