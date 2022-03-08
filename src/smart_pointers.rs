// Using Box<T>

pub fn run_1() {
    let x = 5;
    // let y = &x; -> A refrence pointing to the value of x;
    let y = Box::new(x); // Y is set to be the instance of the Box pointing to a copied vaue of x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining our own Smart pointer

use std::ops::Deref;

// Impementing deref trait.
// function deref borrows self and returns the refrence to the inner data.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn run() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
