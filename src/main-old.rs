extern crate ws;

use std::rc::Rc;
use std::cell::Cell;

use ws::{listen, WebSocket, Handler, Sender, Handshake, Result, Message, CloseCode};

const POOL_PORT: &str = "5545";
// const ETH_PORT:  &str = "8545";

struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {

    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        // WebSocket::broadcaster().send("New Websocket")
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("Got message: {}", msg);
        Ok(())
        // self.out.close(CloseCode::Normal)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }
}

fn main() {
  // Now, instead of a closure, the Factory returns a new instance of our Handler.
  let count = Rc::new(Cell::new(0));
  listen(format!("ws://127.0.0.1:{}", POOL_PORT), |out| { Server { out: out, count: count.clone() } } ).unwrap()
}
