// The move closure is used thread::spawn because it allows you to use data from one thread in another thread.

use std::thread;

pub fn run() {
    let v = vec![1, 2, 3];

    // move keyword forces the closure to take the ownership of the 'v'
    // because if it just borrow 'v' the problem will be, Rust don't know
    // how long the Spawned thread will run, so it doesn't know if the refernce
    // to v will always be valid.
    let handle = thread::spawn(move || {
        println!("There is a vector {:?}", v);
    });

    //drop(v);

    handle.join().unwrap();
}
