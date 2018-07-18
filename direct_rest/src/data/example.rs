use iface::transfer::Transfer;
use serde_json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Example {
    nr: i32,
    text: String,
}

impl Example {
    pub fn new(nr: i32, text: String) -> Self {
        Example { nr, text }
    }
}

impl Transfer for Example {
    fn encode(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode(repr: &str) -> Self {
        serde_json::from_str(repr).unwrap()
    }

    fn clean(&self) {
        // pass
    }
}

#[cfg(test)]
mod tests {
    use super::Example;
    use super::Transfer;

    #[test]
    fn test_example() {
        let mut ex = Example::new(1, "hi".to_owned());
        let mut txt = ex.encode();
        let mut back = Example::decode(&txt);
        assert_eq!(ex, back);
    }
}
