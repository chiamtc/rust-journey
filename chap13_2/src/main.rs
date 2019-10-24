fn main() {
    println!("Hello, world!");

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Got {}", val)
    }
}

/*
The iter method produces an iterator over immutable references.
If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.
*/

#[test]
fn iterator_demonstration(){
    let v1= vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn summation(){
    let v1= vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    assert_eq!(total, 6);
}


#[test]
fn map_test(){
    let v1 = vec![1,2,3];
    //map = iterator adaptor. Iterator by itself is lazy, so collect() to make it in a collection
    let v2:Vec<i32> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2,3,4])
}
