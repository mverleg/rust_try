use enum_types::field::Nr;
use enum_types::func::MyFunctionality;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MyType {
    Alpha(String),
    Beta(i32, i32),
    Gamma(bool, i32, String, Vec<u8>, Nr),
}

impl MyFunctionality for MyType::Alpha {
    fn hello() -> i32 {
        unimplemented!()
    }
}
