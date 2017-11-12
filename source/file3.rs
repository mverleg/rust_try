use std::fmt; // Import the `fmt` module.
use std::f64::consts::PI;

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    let tup = ("M", 28, PI);
    let (a, b, c) = tup;
    println!("{2:} {1:} {0:}", a, b, c);

    let bigarray: [f64; 1000] = [1.; 1000];
    println!("{}", bigarray.iter().fold(0f64, |sum: f64, x: &f64| sum + x));

    let bigarray2: [f64; 1000]; // cannot be used
//    println!("{}", bigarray2[10]);

    #[derive(Debug)]
    struct Point {
        x : i32,
        y : i32,
        z : i32,
    }
    let p : Point = Point {x: 1, y: 2, z: 4};
    println!("{0:?}", p);
}
