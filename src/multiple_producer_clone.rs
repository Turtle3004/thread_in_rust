use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            tx.send(value).unwrap(); //variable moved due to use in closure
            thread::sleep(Duration::from_secs(1));
        }
    });

    //let tx1 = tx.clone(); error -> value borrowed here after move

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for value in values {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved);
        //thread::sleep(Duration::from_secs(1));
    }
}
