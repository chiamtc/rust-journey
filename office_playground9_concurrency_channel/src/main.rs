use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const NUM_TIMERS: usize = 24;


fn timer (d:usize, tx:mpsc::Sender<usize>){
    thread::spawn(move || {
        println!("{}: setting timer ...", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("{}: sent!",d);
        tx.send(d).unwrap()
    });
}

fn main() {
    //timer example
    //multiple producers (tx) and a single receiver (rx)
    let (tx,rx) = mpsc::channel();
    for i in 0..NUM_TIMERS{
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS){
        println!("{}: received!", v);
    }

    //introduction to channel
    /*
    //mpsc = multiple producer single consumer - concept from concurrency in rust
    let (tx,rx) = mpsc::channel();
    //transmitter = tx, pushes message, has send(param) : Resutls<Ok, Err>
    //receiver = rx, receives the message, recv() is a blocking method which blocks the main thread from executing and wait for the message to be passed from tx
    // and try_recv() - non-blocking when we dont need immediate results and let the main thread continues doing their things

    thread::spawn(move || {tx.send(42).unwrap(); });

    println!("got {}", rx.recv().unwrap());

    println!("Hello, world!");*/
}
