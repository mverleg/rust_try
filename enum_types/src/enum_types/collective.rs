use enum_types::field::Nr;
use enum_types::func::MyFunctionality;
use std::cmp::min;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MyType {
    Alpha(String),
    Beta(i32, i32),
    Gamma(bool, i32, String, Vec<u8>, Nr),
}

// NOTE: cannot impl for MyType::Alpha
impl MyFunctionality for MyType {
    fn hello(&self) -> i32 {
        use self::MyType::*;
        match self {
            Alpha(s) => s.len() as i32,
            Beta(a, b) => min(*a, *b),
            Gamma(_, i, s, v, n) => *i as i32 + s.len() as i32 + v.len() as i32 + n.0,
        }
    }
}

pub fn try_assign() {
    // NOTE: cannot use MyType::Alpha as type
    let q: MyType = MyType::Beta(1, 2);
}
