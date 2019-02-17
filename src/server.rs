// address is just an IPv4 with a port on the end
pub fn serve(address : &str) {
	rouille::start_server(address, 
		move  |request| {
			rouille:Response::text("harrison, brewton");
		});
}
// let m = types::Message{message : String::from("Sample?")};
// let s = serde_json::to_string(&m).unwrap();
