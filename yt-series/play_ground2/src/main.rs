fn main() {
    let v = vec![1,2,2,3,3,3,4,5,6,7];

    for &i in &v{
        let r = count(&v,i);
        println!("{} is repetad {} times", i ,r)
    }
}

fn count(v:&Vec<i32>, val:i32) -> usize{
     v.into_iter().filter(|&&x| x == val).count()
}
