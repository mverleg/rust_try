
// https://stackoverflow.com/questions/51162337/implement-function-for-div-rem-types-where-the-result-also-implements-div-rem

use std::ops::{ Rem, Div };

const BINS: i32 = 16;

pub trait Int: Rem + Div + Eq + Sized {}

impl Int for i32 {}
impl Int for usize {}
// also other int types

pub fn stuff<T: Int>(arg: T) -> T {
    (arg / BINS) % BINS
}

pub fn main() {}
