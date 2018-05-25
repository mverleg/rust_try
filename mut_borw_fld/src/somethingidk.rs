struct Ding {
    opt: Option<i32>,
}

impl Ding {
    fn doit(self) -> Option<i32> {
        self.opt
    }
}

enum RorD {
    D(Ding),
    R(Ding),
}

impl RorD {
    #[inline]
    fn as_R(self) -> Self {
        match self {
            RorD::D(ding) => RorD::R(ding),
            r => r,
        }
    }
}

struct Wrap {
    r_or_d: RorD,
}

impl Wrap {
    fn action(&mut self) {}

    fn f(&mut self) {
        let rd = &mut self.r_or_d;
        match rd {
            RorD::D(ref mut d) =>
            RorD::R() => { /* ... */ }
        }
    }
}

fn main() {}
