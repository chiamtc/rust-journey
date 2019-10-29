use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let x = vec![1, 2, 3, 4];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    for i in &v {
        println!("{}", i)
    }

    println!("{:?} {} {}", &v, v.len(), v.capacity());

    println!("{:?}", v.pop().unwrap());

    let r = vec![
        Example::Int(12),
        Example::Float(2.4),
        Example::Text(String::from("abc"))
    ];

    /*  let mut res = Example::Int(0);
      for i in &r {
          res = match i {
              Example::Int(a) => {
                  println!("{} an integer", a);
                  Example::Int(*a)
              }
              Example::Float(a) => {
                  println!("{} a float number", a);
                  continue;
              }
              Example::Text(a) => {
                  println!("{} an actual String", a);
                  continue;
              }
          };
      }
      println!("{:?}", res);
  */

    let mut hm = HashMap::new();
    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);


    for (k, v) in &hm {
        println!("{} , {}", k, v)
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match")
    }

    match hm.get(&String::from("no keys")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match")
    }

    hm.remove(&String::from("strings"));

    let s = Some("c");
    //this
    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }

    //equals to this but without the exhaustive matching like match s{}
    if let Some(i) = s { //Some(i) is the pattern
        println!("{}", i);
    } else {
        {}
    }

    let mut z = Some(0);
    while let Some(i) = z {
        if i > 10 {
            println!("quit");
            z = None;
        } else {
            println!("{}", i);
            z = Some(i + 2)
        }
    }

    let f1 = 24.12312_f32;
    let i_1 = f1 as u8;
    let c_1 = i_1 as char;
    println!("{} {} {}", f1, i_1, c_1);


    let f = File::open("sample.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("paniking and error {:?}", error)
        }
    };
}

/*
enum of Option
enum Option<T>{
    Some<T>,
    None
}

enum of Result
enum Result <T,E>{
    Ok(T),
    Err(E)
}
*/

