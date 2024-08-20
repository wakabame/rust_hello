use std::sync::mpsc;
use std::thread;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // receiver をスレッドに送って, メッセージを受診できる状態にする
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello World!");

    let _ = handle.join();
}
