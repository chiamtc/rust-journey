pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String{
    format!("Hello {}", name)
}

//#[cfg(target_os = "linux")] //only run this func on linux
//#![allow(dead_code)] //not really useful

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2,2));
    }

    fn greeting_test(){
        let a = greeting("Alice");
        assert!(a.contains("Alice"));
    }

    #[test]
    fn check_two() {
        assert!(1 + 1 == 2);
    }

    #[test]
    #[should_panic]
    fn check_three() {
        assert!(1 + 1 != 2);
    }
}

fn main() {
    println!("Hello, world!");
}

//to run a single test cargo test check_two