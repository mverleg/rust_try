
use std::fmt;

mod basics {
    //! Documentation for this module
    //  Just a comment
//    pub fn find_even(vector: &[int]) -> Option<int> {
//        for vector.each |x| {
//            if *x % 2 == 0 {
//                return Some(*x);
//            }
//        };
//        None
//    }
}

fn main() {
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "{0:}", self.0)
        }
    }
    let q : i32 = 4;
    for x in 0..10 {
        println!("nr {0:}", x);
    }
    println!("Hello {name:}", name="Mark");

}


