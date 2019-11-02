/*
trait Iterator{
    type Item; //the item to be an iterable

    fn next(&mut self) -> Option<Self::Item>;
}
*/


fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    let v = vec![1, 2, 3];
    v.iter().next();

    let s:u32 =
        (0..).map(|n| n*n)
            .take_while(|&n| n< 10000)
            .filter(|&n| is_even(n))
            .fold(0, |s,i| s+1);
    println!("{}",s)
}
