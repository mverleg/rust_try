
use iface::restable::Restable;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    nr: i32,
    text: String,
}

impl Restable for Example {
    fn encode(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode(repr: &str) -> Self {
        serde_json::from_str(repr).unwrap()
    }

    fn clean(&self) {
        // pass
    }
}
