use std::sync::mpsc;
use std::thread;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("hi");
        tx.send(value).unwrap();
        //println!("value is {}", value);
    });

    let recieved = rx.recv().unwrap();
    println!("Got value : {}", recieved);
}
