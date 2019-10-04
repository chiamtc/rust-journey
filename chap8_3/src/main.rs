use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    /*for score in scores{
        println!("{}", score)
    }*/

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10,50];
    let scores2:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favourite Color");
    let field_value = String::from("Red");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); //can't use field_name and field_value anymore due to ownership being copied into HashMap(k,v)


    let color = String::from("red");
    let c =scores.get(&color);
    println!("{:?}",c); //returns Option<T>

    //looping hashmap
   /* for (key,value) in scores{
        println!("{} , {} " , key, value);
    }*/

    //updating hashmap
    scores.insert(String::from("blue"), 60);
   /* for (key,value) in scores{
        println!("{} , {} " , key, value);
    }
*/
    //only inserting a value if the key has no value \using Entry API (a value that might or might not exist)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text:&str= "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1; //deference to actual value for manipulation
        //https://stackoverflow.com/questions/40531912/what-is-the-usage-of-the-asterisk-symbol-in-rust
    }
}
