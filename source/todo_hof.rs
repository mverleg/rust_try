
//! http://rustbyexample.com/fn/hof.html

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let upper = 1000;
    let vals: [i32; upper] = [0; upper];
    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // All natural numbers squared
             .take_while(|&n| n < upper) // Below upper limit
             .filter(|&n| is_odd(n))     // That are odd
             .fold(0, |sum, i| sum + i); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
