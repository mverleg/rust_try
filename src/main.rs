
struct Scope {
    value: i32
}
impl Scope {
    fn self_and_return<'a>(&'a self, inp: &'a i32) -> &'a i32 {
        if inp > &self.value {
            return inp
        } else {
            return &self.value
        }
    }
}

fn main() {
    // Notes:
    // * Lifetime annotations are required on self_and_return, because the default lifetime is self
    // * Neither p nor s can be moved within the x-scope, otherwise borrows expire
    println!("Hello, world!");
    let p = 5;
    let s = Scope { value: 7 };
    let x;
    {
        x = s.self_and_return(&p);
    }
    println!("x = {}", x)
}
