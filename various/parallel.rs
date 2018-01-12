
use std::thread;
use std::sync::mpsc;

// Count to 50 by passing 1 to a thread, which passes x+1 to the next, which passes x+1 to the
// next, all in a big snake until 50 comes out. Complexity is only O(n) but takes long somehow...
pub fn main() {
    let (tx, mut rxold) = mpsc::channel();
    tx.send(1).unwrap();
    for _ in 1 .. 50 {
        let (tx, rxnew) = mpsc::channel();
        thread::spawn( move || {
            let val = rxold.recv().unwrap();
            tx.send(val + 1).unwrap();
        });
        rxold = rxnew;
    }
    println!("final value: {}", rxold.recv().unwrap());
}


