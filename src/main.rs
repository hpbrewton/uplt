mod types;
mod client;

fn main() {
    for a in std::env::args() {
        if a == "server" {
            server();
        }
        if a == "client" {
            client::client();
        }
    }
	let m = types::Message{message : String::from("Sample?")};
	let s = serde_json::to_string(&m).unwrap();

	println!("serialized = {}", s);
}

fn server() {
}
