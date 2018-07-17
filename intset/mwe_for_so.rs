#![feature(specialization)]

// https://stackoverflow.com/questions/51162337/implement-function-for-div-rem-types-where-the-result-also-implements-div-rem

use std::ops::{Rem, Div};

const BINS: u8 = 16;

pub trait Int: Rem + Div + Eq + Sized {}

impl Int for i32 {}

impl Int for usize {}
// also other int types

//pub fn stuff<T>(arg: T) -> T where T: Div<T>, T::Output: Rem<T> + From<i32> {
//    (arg / BINS.into()) % BINS.into()
//}

//pub fn convert<T>(arg: T) -> usize where T: Into<usize> {
//    arg.into()
//}

//trait MyFrom<T> {
//    fn my_from(val: T) -> Self;
//}
//
//impl<T, U: From<T>> MyFrom<T> for U {
//    default fn my_from(val: T) -> U {
//        U::from(val)
//    }
//}

//impl MyFrom<i8> for u8 {
//    default fn my_from(val: i8) -> u8 {
//        val as u8
//    }
//}

//pub fn conv<T>(arg: i32) -> T {
//where T: Div<i32>, T::Output: Rem<i32, Output = T>
    //pub fn conv<T>(arg: T) -> T where T: From<i32> + Div<T>, T::Output: T {
pub fn stuff<T>(arg: T) -> T where T: From<u8> + Div<T, Output = T> + Rem<T, Output = T> + Clone {
    (T::from(BINS) / arg.clone()) % arg
}

pub fn main() {
    stuff(1i16);
    stuff(1i32);
    stuff(1i64);
    stuff(1u8);
    stuff(1u16);
    stuff(1u32);
    stuff(1u64);
    stuff(1usize);
}
