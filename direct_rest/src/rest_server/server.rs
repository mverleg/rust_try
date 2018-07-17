extern crate simple_server;

use self::simple_server::Server;

fn main() {
    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello Rust!".as_bytes().to_vec())?)
    });

    server.listen("127.0.0.1", "8888");
}
