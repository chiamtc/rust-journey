/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


#[derive(Debug)]
struct User {
    firstName: String,
    lastName: String,
    age: u32,
}

impl User{
    fn is_larger<'a>(&'a self, user2:&'a User) -> &'a User{
        if &self.age > &user2.age {
//            User { firstName:self.firsName, lastName:self.lastName, age:self.age}
           self
        }else {
            user2
        }
    }

    fn editage(&mut self, incrementAge:u32) -> &User{
        self.age = &self.age + incrementAge;
        self
    }

    fn larger(self, user2:User) -> User{
        if self.age > user2 .age {
            self
        }else{
            user2
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 32 };

    let mut user1 = User { firstName: String::from("Tat"), lastName: String::from("Cheng"), age: 14 };
    let user2 = User { firstName: String::from("Tat"), lastName: String::from("Cheng"), age: 13 };
//    println!("{:?}",user1.is_larger(&user2));
    println!("{:?}", user1.editage(1));
    println!("{:?}",user1.larger(user2));
//    println!("user details {:?}", user1);

//    println!("rect1 area is: {:?} ", rect1);
}
