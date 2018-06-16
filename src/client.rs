// Include Extern Modules:
use serde_json;
use ws;

// Include Standard Modules:
use std::thread;

// Constants:
const MAX_ITEMS: usize = 100;
const MAX_MESSAGES: usize = 600;

struct ClientHandler {
    out: ws::Sender,
    count: usize
}

impl ClientHandler {
    fn new(out: ws::Sender) -> Self {
        Self { out, count: 0 }
    }

    fn send_data(&mut self) {
        let out = self.out.clone();
        // Generate data:
        let data = (0..MAX_ITEMS).map(|i| json!({
            "id": "95FF6D82",
            "num1": i,
            "num2": i % 2,
            "num3": i % 3,
            "word": "test",
        })).collect::<Vec<serde_json::Value>>();
        let message = ws::Message::Text(serde_json::to_string(&data).unwrap());

        self.count = MAX_MESSAGES;
        thread::spawn(move || {
            for i in 1..=MAX_MESSAGES {
                println!("Client: Send Message {}", i);
                out.send(message.clone()).unwrap();
            }
        });
    }
}

impl ws::Handler for ClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.send_data();
        Ok(())
    }

    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        self.count -= 1;
        println!("Client: Recv Message, {} remaining", self.count);
        if self.count == 0 {
            self.out.shutdown().unwrap();
        }
        Ok(())
    }

    fn on_error(&mut self, err: ws::Error) {
        // We should get a Queue error on >500 queued, but don't:
        println!("Client: {:?}", err);
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("Client: Closed");
    }
}

pub fn run() {
    // Setup websocket client:
    ws::connect("ws://127.0.0.1:2000", |out| {
        println!("Client: Connected");

        ClientHandler::new(out)
    }).unwrap();
}