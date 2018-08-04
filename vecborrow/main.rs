
#[derive(Debug)]
pub struct Q(i32);

pub fn f<'a>(x: &'a Vec<Q>) -> Vec<&'a Q> {
    vec![&x[0], &x[2], &x[4]]
}

pub fn main() {
    let my_data = vec![Q(1), Q(1), Q(2), Q(3), Q(5), Q(8), Q(13)];
    {
        let brwd: Vec<&Q> = f(&my_data);
        println!("{:?}", brwd);
    }
    println!("{:?}", my_data);
}
