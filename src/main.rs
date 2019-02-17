
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
	message : std::string::String,
}

struct Index {
	index: u64,
}

fn main() {
	let m = Message{message : String::from("Sample?")};
	let s = serde_json::to_string(&m).unwrap();
	
	println!("serialized = {}", s);
}