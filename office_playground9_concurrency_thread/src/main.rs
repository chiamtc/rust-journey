use std::thread;
fn main() {
   /* let mut c = vec![];

    for i in 0..10{
        c.push(thread::spawn(move ||{
            println!("thread number {} ", i)
        }))
    }
    for j in c{
        j.join();
    }*/

    let v = vec![1,2,3];

    //reason to use move is to allow the variable in closure function to be forced to use own the ownership of v
    let handle = thread::spawn( move || {
        println!("vector: {:?}", v);
    });
    handle.join();
}
