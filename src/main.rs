mod types;
mod client;

fn main() {
    for a in std::env::args() {
        if a == "server" {
            server();
        }
        if a == "client" {
            client::client()
        }
    }
}

fn server() {
    //server::serve("0.0.0.0:8080");
}
