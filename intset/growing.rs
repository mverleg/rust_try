
use std::ops::{Rem, Div};

const BINS: u8 = 16;

enum Bin<T> {
    Empty,
    Value(T),
}

pub struct IntSet<T> where T: Clone + From<u8> + Into<usize> + Rem<T, Output = T> + PartialEq<T> {
    bins: Vec<Bin<T>>,
}

impl<T> IntSet<T> where T: Clone + From<u8> + Into<usize> + Rem<T, Output = T> + PartialEq<T> {
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
    pub fn add(&mut self, value: T) {
        // (T::from(BINS) / arg.clone()) % arg
        let indx: usize = (value.clone() % T::from(BINS)).into();
        let inbin = &mut self.bins[indx];
        match inbin {
            Bin::Empty => *inbin = Bin::Value(value),
            Bin::Value(ref existing) => assert!(existing == &value),
        }
//        T::from(BINS);
    }
}

pub fn main() {
//    let mut set = IntSet::new();
//    set.add(1i16);
//    let mut set = IntSet::new();
//    set.add(1i32);
//    let mut set = IntSet::new();
//    set.add(1i64);
    let mut set = IntSet::new();
    set.add(1u8);
    let mut set = IntSet::new();
    set.add(1u16);
    let mut set = IntSet::new();
//    set.add(1u32);
//    let mut set = IntSet::new();
//    set.add(1u64);
//    let mut set = IntSet::new();
    set.add(1usize);
}
