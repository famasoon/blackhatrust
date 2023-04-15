use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let pointer = Arc::new(Mutex::new(5));
    let second_pointer = pointer.clone();
    thread::spawn(move || {
        let mut mutable_pointer = second_pointer.lock().unwrap();
        *mutable_pointer = 1;
    });
    thread::sleep(time::Duration::from_secs(1));
    let one = pointer.lock().unwrap();
    println!("{}", one);
}
