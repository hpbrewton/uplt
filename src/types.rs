use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
	  pub message : std::string::String,
}

#[derive(Serialize, Deserialize)]
struct Index {
	  index: u64,
}
