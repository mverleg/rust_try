
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
        let mut doitres: Option<&mut Option<i32>> = None;
        {
            let rd = &mut self.r_or_d;
            match rd {
                RorD::D(ref mut d) => {
                    doitres = Some(d.doit());
                }
                RorD::R() => { /* ... */ }
            }
        }
        if let Some(res) = doitres {
            match res {
                Some(nr) => { /* ... */ }
                None => { self.action(); }
            }
        }
    }
}

fn main() {
}
