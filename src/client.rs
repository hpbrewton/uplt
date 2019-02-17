use reqwest::*;


pub fn client() {
    println!("Client");
    let client = Client::new();
    let resp = client .get("http://httpbin.org") .body("body").send();
    match resp {
        Ok(ok) => println!("OK {:?}", ok),
        Err(err) => println!("ERR {:?}", err),
    }

}
