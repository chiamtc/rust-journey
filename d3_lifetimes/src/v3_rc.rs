use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
pub struct WithLife<'a> {
    s: &'a String,
}

#[derive(Debug)]
pub struct NoLife{
//    s:Rc<String>
    s:Rc<RefCell<String>>
}

/*
fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    Ok((WithLife { s: &s }, WithLife { s: &s }))
}
*/

fn main() -> Result<(), std::io::Error>{
//    let (l1,l2) = make_with_life("test_data/v3_data.txt")?;
    let (l1,l2) = make_no_life("test_data/v3_data.txt")?;
    let mut s = l1.s.borrow_mut();
//    let s2 = l2.s.borrow(); //will panic because it's being borrow in let mut s.
    s.push_str("what if it was even bigger");
    println!("{:?}", l1);
    println!("{:?}", l2);
    println!("s == {}", s);
    drop(s);
    println!("{:?}", l1);
    println!("{:?}", l2);
    Ok(())
}

fn make_no_life(fname:&str) -> Result<(NoLife, NoLife), std::io::Error>{
    let s = std::fs::read_to_string(fname)?;
//    let r = Rc::new(s);
    let r = Rc::new(RefCell::new(s));
    Ok((NoLife{s:r.clone()}, NoLife{s:r.clone()}))
}
