fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //using if in a let statement
    let condition = true;
    let number2 = if condition { //the value of the whole if expression depends on which block of code executes.
        5
    } else {
        6
    };
    println!("The value of number is: {}", number2);

    //loop
    //keeps looping until user press ctrl+c
    /*loop{
        println!("keeps looping");
    }*/

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //while loop
    let mut number3 = 3;

    while number3 != 0 {
        println!("{}!", number3);

        number3 -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() { //iterating each element in array
        println!("the value in for loop is {}", element);
    }

    for (number, index) in (1..4).rev().enumerate() { //reverse() enumerate() = get position
        println!("the value in rev() {} at position {}", number, index)
    }
}
