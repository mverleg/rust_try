#![feature(nll)]

use std::ops::{Rem, Div};

const BINS: u8 = 16;

enum Bin<T> where T: Clone + From<u8> + Into<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T> {
    Empty,
    Value(T),
    Sub(IntSet<T>),
}

pub struct IntSet<T> where T: Clone + From<u8> + Into<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T> {
    bins: Vec<Bin<T>>,
}

impl<T> IntSet<T> where T: Clone + From<u8> + Into<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T> {
    pub fn new() -> Self {
        let mut bins = Vec::<Bin<T>>::with_capacity(BINS.into());
        for _ in 0..BINS {
            bins.push(Bin::Empty);
        }
        IntSet {
            bins: bins
        }
    }

    pub fn add(&mut self, value: T) -> bool {
        // Can I prevent this clone for those tyoes that are Copy? Or can I assume the optimizer takes care of it?
        let indx: usize = (value.clone() % T::from(BINS)).into();
        let inbin = &mut self.bins[indx];
        match inbin {
            Bin::Empty => {
                // Insert the value.
                *inbin = Bin::Value(value);
                true
            }
            Bin::Value(ref existing) => {
                if existing == &value {
                    // The value already exists, nothing to do.
                    false
                } else {
                    // There is a collision, add a level (stripping bits happens in the recursed call).
                    let mut subset = IntSet::new();
                    subset.add(existing.clone());
                    subset.add(value);
                    *inbin = Bin::Sub(subset);
                    true
                }
            }
            Bin::Sub(subset) => {
                // Pass on to the next level (stripping some bits of the number).
                subset.add(value / T::from(BINS))
            },
        }
    }
}

// TODO @mverleg: copy tests
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
