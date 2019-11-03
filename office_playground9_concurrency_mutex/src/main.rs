use std::sync::{Mutex, Arc};
use std::thread;
//mutex = mutual exclusion
// one thread to access data at one time.
// the thread that needs to access has to acquire the lock of the mutex
//once it's done, you need to unlock it so that other thread can access to it.
// also act as a smart pointer, return mutex guard which implements deference ()


fn main() {
    //Arc = is an atomically referenced counter type,
    // arc converts  the type int primitive type which safe to share across threads.
    let c = Arc::new(Mutex::new(0));
    let  mut hs = vec![];

    /*
    Whole sequence of this loop =
    1. loop 10 times
    2. each loop spawns a thread and gain the access of the mutex and increase the value by 1
    3. next thread does the same thing in 2.
    */
    for _ in 0..10{
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            //locking the access exclusively for the thread at this instance
            let mut num = c.lock().unwrap();
            //smart pointer here, so we need to deference it to access the value
            *num +=1;
            println!("{}", num);
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }
    println!("Result: {}" , *c.lock().unwrap());
}
