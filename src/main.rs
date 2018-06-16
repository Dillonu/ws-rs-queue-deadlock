// Import Extern Crates:
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate ws;

// Include Standard Modules:
use std::thread;

// Include Local Modules:
mod client;
mod server;

fn main() {
    // Spawn Server:
    let server_thread = thread::spawn(move || {
        println!("Server: Start");
        server::run();
        println!("Server: End");
    });
    // Spawn Client:
    let client_thread = thread::spawn(move || {
        println!("Client: Start");
        client::run();
        println!("Client: End");
    });

    // Join thread:
    client_thread.join().unwrap();
    server_thread.join().unwrap();
}
