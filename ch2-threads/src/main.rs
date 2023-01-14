// chapter2 / threads

use std::thread;

fn main() {
    for i in 1..10 {
        let handle = thread::spawn(move || {
            println!("Hello from a thread! {}", i);
        });
        let _ = handle.join();
    }
}
