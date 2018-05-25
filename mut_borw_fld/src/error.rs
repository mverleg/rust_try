
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

struct Wrap {
    r_or_d: RorD,
}

impl Wrap {
    fn action(&mut self) {}

    fn f(&mut self) {
        let rd = &mut self.r_or_d;
        match rd {
            RorD::D(ref mut d) => {
                let res = d.doit();
                // I would like to end borrow of `d` or `rd` here
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
