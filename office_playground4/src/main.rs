use std::collections::HashMap;

#[derive(Debug)]
enum Example{
    Float(f64),
    Int(i32),
    Text(String)
}

fn main() {
    let x = vec![1,2,3,4];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    for i in &v{
        println!("{}", i)
    }

    println!("{:?} {} {}", &v, v.len(), v.capacity());

    println!("{:?}",v.pop().unwrap());

    let r = vec![
        Example::Int(12),
        Example::Float(2.4),
        Example::Text(String::from("abc"))
    ];

    let mut res= Example::Int(0);
    for i in &r{
        res = match i{
            Example::Int(a) => {
                println!("{} an integer", a);
                Example::Int(*a)
            },
            Example::Float(a) => {
                println!("{} a float number",a);
                continue;
            },
            Example::Text(a) => {
                println!("{} an actual String", a);
                continue;
            }
        };
    }
    println!("{:?}", res);


    let mut hm = HashMap::new();
    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);


    for (k,v) in &hm{
        println!("{} , {}", k,v)
    }

    match hm.get(&String::from("random")){
        Some(&n) => println!("{}",n),
        _ => println!("no match")
    }

    hm.remove(&String::from("strings"));


}


