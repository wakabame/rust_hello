use std::thread;

pub fn main() {
    thread::spawn(|| {
        println!("Hello, world!");
    });
}
