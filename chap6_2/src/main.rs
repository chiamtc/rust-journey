enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum UsState {
    Alabama,
    Alaska,
    Connecticut,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
/*
not working
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}*/

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter;
    println!("{}", value_in_cents(coin1));
    println!("{}", value_in_cents(coin2));
    println!("{}", value_in_cents(coin3));
    println!("{}", value_in_cents(coin4));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // special character =  _ , placeholder like default in switch()
    let some_u8_value = 0u8;
    match some_u8_value{
        1 => println!("one"),
        _ => println!("not a value bruh")
    }

    some_u8_value;
}
