use std::thread;
use std::time::Duration;

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread has to wait for the spawned thread to finish and then run its for loop.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hello number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // The main thread has to wait for the spawned thread to finish before it can exit.
    handle.join().unwrap();
}
