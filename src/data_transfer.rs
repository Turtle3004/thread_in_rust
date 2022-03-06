// “Do not communicate by sharing memory; instead, share memory by communicating.”

use std::sync::mpsc;
use std::thread;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    //println!("Hello from the Mars!");

    // recv() will block the main thread execution and wait until a value is sent down the channel.
    // try_recv() doesn't block but will return a Result<T, E> immediately.
    // try_recv() is used if the thread has other work to do while waiting for the message.

    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);
}
