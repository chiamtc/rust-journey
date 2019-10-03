fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v2 = vec![1,2,3,4,5];
    let third: &i32 = &v2[2];

    println!("{}", third);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    for i in &v2 {
        println!("{}", i)
    }

    ///////
    let mut v3 = vec![2,3,4];
    for i in &mut v3{
        *i +=50;
    }

    for i in &v3 {
        println!("{}", i)
    }
    //////////


    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.2),
        SpreadsheetCell::Text(String::from("321312"))
    ];



    for i in &row {
        println!("{:?}", i)
    }
}
