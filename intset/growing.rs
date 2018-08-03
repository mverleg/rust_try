#![feature(nll, try_from, try_info, specialization)]

use std::ops::{Rem, Div};
use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Add;

const BINS: u8 = 16;

/// Define a modulo operation, in the mathematical sense.
/// This differs from Rem because the result is always non-negative.
trait Modulo<T> {
    type Output;

    #[inline]
    fn modulo(&self, other: &T) -> Self::Output;
}

/// Implement modulo operation for types that implement Rem, Add and Clone.
// Add and Clone are needed to shift the value by U if it is below zero.
// TODO @mverleg: describe constraint on output type
impl<U, T> Modulo<T> for U
    where
        T: Clone,
        U: Rem<T> + Clone,
        <U as Rem<T>>::Output: Add<U>,
        <<U as Rem<T>>::Output as Add<U>>::Output: Rem<T>,
    {
    type Output = <<<U as Rem<T>>::Output as Add<U>>::Output as Rem<T>>::Output;

    #[inline]
    default fn modulo(&self, other: &T) -> Self::Output {
        ((self.clone() % other.clone()) + self.clone()) % other.clone()
    }
}

/// Implement a potentially faster modulo operation for types that are Copy-able.
impl<U, T> Modulo<T> for U
    where
        T: Clone + Copy,
        U: Rem<T> + Clone + Copy,
        <U as Rem<T>>::Output: Add<U>,
        <<U as Rem<T>>::Output as Add<U>>::Output: Rem<T>,
    {

    #[inline]
    fn modulo(&self, other: &T) -> Self::Output {
        ((*self % *other) + *self) % *other
    }
}


#[derive(Debug)]
enum Bin<T> where
        // todo: remove Debug
        T: Debug + Clone + TryFrom<u8> + TryInto<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T>,
        <T as TryFrom<u8>>::Error: Debug, <T as TryInto<usize>>::Error: Debug {
    Empty,
    Value(T),
    Sub(IntSet<T>),
}

#[derive(Debug)]
pub struct IntSet<T> where
        T: Debug + Clone + TryFrom<u8> + TryInto<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T>,
        <T as TryFrom<u8>>::Error: Debug, <T as TryInto<usize>>::Error: Debug {
    bins: Vec<Bin<T>>,
}

impl<T> IntSet<T> where
        T: Debug + Clone + TryFrom<u8> + TryInto<usize> + Rem<T, Output=T> + Div<T, Output=T> + PartialEq<T>,
        <T as TryFrom<u8>>::Error: Debug, <T as TryInto<usize>>::Error: Debug {
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
        // Hopefully the compiler takes care of removing this 'clone'.
        let indx: usize = (value.clone() % T::try_from(BINS).unwrap()).try_into().unwrap();
        let seekbin = &mut self.bins[indx];
        match seekbin {
            Bin::Empty => {
                // Insert the value.
                *seekbin = Bin::Value(value);
                true
            }
            Bin::Value(ref existing) => {
                if existing == &value {
                    // The value already exists, nothing to do.
                    false
                } else {
                    // There is a collision, add a level (stripping bits happens in the recursed call).
                    let mut subset = IntSet::new();
                    subset.add(existing.clone() / T::try_from(BINS).unwrap());
                    subset.add(value / T::try_from(BINS).unwrap());
                    *seekbin = Bin::Sub(subset);
                    true
                }
            }
            Bin::Sub(subset) => {
                // Pass on to the next level (stripping some bits of the number).
                subset.add(value / T::try_from(BINS).unwrap())
            },
        }
    }

    pub fn contains(&self, value: T) -> bool {
        println!("");
        println!("value {:?}", value.clone());
        println!("bins  {:?}", BINS);
        println!("from  {:?}", T::try_from(BINS));
        println!("mod   {:?}", value.clone() % T::try_from(BINS).unwrap());
        println!("into  {:?}", (value.clone() % T::try_from(BINS).unwrap()).try_into());
        let indx: usize = (value.clone() % T::try_from(BINS).unwrap()).try_into().unwrap();
        let seekbin = &self.bins[indx];
        match seekbin {
            Bin::Empty => {
                // The value was not found.
                false
            }
            Bin::Value(ref existing) => {
                // Found a value, found iff it matches
                existing == &value
            }
            Bin::Sub(subset) => {
                // Pass on to the next level
                subset.contains(value / T::try_from(BINS).unwrap())
            },
        }
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::IntSet;
    use std::ops::RangeBounds;

    #[test]
    fn test_int_set_types() {
        let set: IntSet<i8> = IntSet::new();
        let set: IntSet<i16> = IntSet::new();
        let set: IntSet<i32> = IntSet::new();
        let set: IntSet<i64> = IntSet::new();
        let set: IntSet<u8> = IntSet::new();
        let set: IntSet<u16> = IntSet::new();
        let set: IntSet<u32> = IntSet::new();
        let set: IntSet<u64> = IntSet::new();
        let set: IntSet<usize> = IntSet::new();
    }

    #[test]
    fn test_int_set_basic() {
        let mut set = IntSet::new();
        for k in -142 .. 142 {
            set.add(k);
        }
        for k in -142 .. 142 {
            assert!(set.contains(k));
        }
        for k in -1024i32 .. 0i32 {
            assert!(!set.contains(k));
        }
        for k in 142 .. 1024 {
            assert!(!set.contains(k));
        }
//        assert_eq!(set.count(), 142);
    }

    #[test]
    fn test_int_set_repeats() {
        let mut set = IntSet::new();
        for k in -142 .. 142 {
            set.add(k);
        }
        for k in -142 .. 142 {
            assert!(!set.add(k));
        }
//        assert_eq!(set.count(), 142);
    }

    #[test]
    fn test_int_set_collisions() {
        let mut set: IntSet<i32> = IntSet::new();
        // To have a value in the first bin for 6 levels, the last 24 bits should be the same - 0 here.
        // To let all the lower levels be created, one must add at least 6 values.
        for k in 1 .. 9 {
            assert!(set.add(k * 2i32.pow(24)));
        }
        for k in 1 .. 9 {
            assert!(set.contains(k * 2i32.pow(24)));
        }
        for k in -142 .. 142 {
            assert!(!set.contains(k));
        }
        assert!(!set.contains(2i32.pow(23)));
//        assert_eq!(set.count(), 8);
    }
}
