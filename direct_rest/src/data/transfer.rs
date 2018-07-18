pub trait Transfer {
    fn encode(&self) -> String;

    fn decode(repr: &str) -> Self;

    fn clean(&self);
}
