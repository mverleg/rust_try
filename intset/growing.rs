
use std::ops::{Rem, Div};
use std::marker::PhantomData;

const BINS: u8 = 16;

struct Bin<T> where T: From<u8> {
    val: PhantomData<T>
}

struct IntSet<T> where T: From<u8> {
    bins: Vec<Bin<T>>,
}

impl<T> IntSet<T> where T: From<u8> {
    pub fn new() -> Self {
        let bins = Vec::<Bin<T>>::with_capacity(BINS.into());
        IntSet {
            bins: bins
        }
    }

//    pub fn stuff(&self, arg: T) -> T where Z: From<u8> + Div<Z, Output=Z> + Rem<Z, Output=Z> + Clone {
    pub fn stuff(&self, arg: T) -> T {
        // (T::from(BINS) / arg.clone()) % arg
        T::from(BINS)
    }
}

pub fn main() {
    let mut set = IntSet::new();
    set.stuff(1i16);
    let mut set = IntSet::new();
    set.stuff(1i32);
    let mut set = IntSet::new();
    set.stuff(1i64);
    let mut set = IntSet::new();
    set.stuff(1u8);
    let mut set = IntSet::new();
    set.stuff(1u16);
    let mut set = IntSet::new();
    set.stuff(1u32);
    let mut set = IntSet::new();
    set.stuff(1u64);
    let mut set = IntSet::new();
    set.stuff(1usize);
}
