
// http://lucumr.pocoo.org/2012/10/18/such-a-little-thing/

fn find_even<T: Copy num::Num cmp::Eq>(vec: &[T]) -> Option<T> {
    let zero: T = num::from_int(0);
    let two: T = num::from_int(2);
    for vec.each |x| {
        if *x % two == zero {
            return Some(*x);
        }
    }
    return None;
}
