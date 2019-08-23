struct User{
    username:String, // &str using ref because we want the data to be valid as long as struct is. not solved here because of lifetime and all in Chap 10
    email:String,
    sign_in_count:u64,
    active:bool
}
fn main() {
    println!("Hello, world!");
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("another@example.com");

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2 //spread operator like js
    };

    //aka tuple struct because ther'es no name struct name
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    struct UnitLikeStruct();
    // Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
}
