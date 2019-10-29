#[derive(Debug)]
struct Object{
    width:u32,
    height:u32
}

fn main() {
    println!("Hello, world!");

    let c = true;
    let n = if c{
        50
    }else{
        76
    };
    println!("{}", n);

    let mut d = 0;
    loop{
        println!("finite");
        d = d +1;
        if d >= 10 {
            break;
        }
    }

   /* 'a :loop{
        println!("loop a");
        'b: loop{
            println!("loop b");
            'c : loop{
                println!("loop c");
                break
            }`
        }
        continue 'a
    }*/

    let y = loop{
        break 10;
    };

    println!("{}", y);

    let mut n = 10;
    while n != 0{
        n =n-1;
    }
    println!("{}",n);
    

    let e = vec![10,20,30,40,50];
    for i in e{
        println!("{}", i)
    }

    for i in 1..100{
        println!("{}", i)
    }

    let x = 5;

    match x {
        1 => println!("1"),
        5=> println!("5"),
        _ => println!("nothing")
    }

    let m = 15;
    match m {
        1 => println!("one"),
        2 | 3 | 4 | 5 | 7 |11 => println!("this is a prime number"),
        13...19 => println!("A teen"),
        _ => println!("old dude")
    }

    let pair = (0, -2);

    match pair {
        (0, y) => println!("y: {}",y),
        (x,0)=> println!("x:{} ", x),
        _ => println!("no match")
    }

    let pair2 = (5,-5);

//known as guard
    match pair2 {
        (x,y) if x == y => println!("identical"),
        (x,y) if x+y == 0 => println!("identical and negative"),
        (x,_) if x% 2 == 0 => println!("left operator is divisible by 0"),
        _ => println!("sorry, nothing matches")
    }


    let p = 5;

    let found = match p {
        p@ 1...12  => Object { width: 1, height: 2},
        p @ 13 ... 19 => Object { width:13, height:14 },
        _ => Object {width:0, height:0}
    };

    println!("{:?}", found)
}
