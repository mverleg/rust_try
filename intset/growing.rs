
use std::ops::{Rem, Div};

const BINS: u8 = 16;

enum Bin<T> {
    Empty,
    Value(T),
}

pub struct IntSet<T> where T: From<u8> + Rem<T> {
    bins: Vec<Bin<T>>,
}

impl<T> IntSet<T> where T: From<u8> + Rem<T> {
    pub fn new() -> Self {
        let mut bins = Vec::<Bin<T>>::with_capacity(BINS.into());
        for _ in 0 .. BINS {
            bins.push(Bin::Empty);
        }
        IntSet {
            bins: bins
        }
    }

//    pub fn stuff(&self, arg: T) -> T where Z: From<u8> + Div<Z, Output=Z> + Rem<Z, Output=Z> + Clone {
    pub fn add(&self, value: T) {
        // (T::from(BINS) / arg.clone()) % arg
        let mut indx: usize = (value % T::from(BINS)).into();
        match self.bins[indx] {

        }
//        T::from(BINS);
    }
}

pub fn main() {
    let mut set = IntSet::new();
    set.add(1i16);
    let mut set = IntSet::new();
    set.add(1i32);
    let mut set = IntSet::new();
    set.add(1i64);
    let mut set = IntSet::new();
    set.add(1u8);
    let mut set = IntSet::new();
    set.add(1u16);
    let mut set = IntSet::new();
    set.add(1u32);
    let mut set = IntSet::new();
    set.add(1u64);
    let mut set = IntSet::new();
    set.add(1usize);
}
