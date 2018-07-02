
#![feature(generators, generator_trait)]
// use std::ops::{Generator, GeneratorState};
use std::ops::{Generator};
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
struct Alpha {}

impl Alpha {
    fn new() -> Self { Alpha {} }
}

trait Beta: Debug {
    fn go(&self);
}

impl Beta for Alpha {
    fn go(&self) {}
}

fn makegen() -> Box<Generator<Yield=i32, Return=()>> {
    Box::new(|| {
        println!("hello world");
        yield 42
    })
}

fn main() {
    let reader: Rc<RefCell<Beta>> = Rc::new(RefCell::new(Alpha::new()));
    println!("{:?}", reader.borrow_mut());
    println!("{:?}", reader.borrow_mut().go());
}

