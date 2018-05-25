
struct Ding {
    opt: Option<i32>,
}

impl Ding {
    fn doit(&mut self) -> &mut Option<i32> {
        &mut self.opt
    }
}

enum RorD {
    D(Ding),
    R(),
}

impl RorD {
    fn action(&mut self) {}

    fn f(&mut self) {
        match self {
            RorD::D(ref mut d) => {
                let res = d.doit();
                // I would like to end borrow of `d` here
                match res {
                    Some(nr) => { /* ... */ }
                    None => { self.action(); }
                }
            }
            RorD::R() => { /* ... */ }
        }
    }
}

fn main() {
}
