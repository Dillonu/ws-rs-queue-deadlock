// Include Extern Modules:
use serde_json;
use ws;

struct ServerHandler {
    out: ws::Sender,
    count: usize
}

impl ServerHandler {
    fn new(out: ws::Sender) -> Self {
        Self { out, count: 0 }
    }
}

impl ws::Handler for ServerHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // Parse data:
        let _: serde_json::Value = serde_json::from_str(&match msg {
            ws::Message::Text(s) => s,
            ws::Message::Binary(b) => String::from_utf8(b).unwrap()
        }).unwrap();

        // Send ack:
        self.count += 1;
        println!("Server: New Message {:?}", self.count);
        self.out.send(ws::Message::Text(String::from("Ack"))).unwrap();

        Ok(())
    }

    fn on_error(&mut self, err: ws::Error) {
        // We should get a Queue error on >500 queued, but don't:
        println!("Server: {:?}", err);
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("Server: Connection Closed");
    }
}

pub fn run() {
    // Setup websocket server:
    ws::listen(("0.0.0.0", 2000), |out| {
        println!("Server: New Connection");

        ServerHandler::new(out)
    }).unwrap();
}