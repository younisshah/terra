extern crate ws;
extern crate nazar;
extern crate crossbeam;

use self::ws::{listen, CloseCode, Sender, Handler, Handshake, Message, Result};
use self::nazar::t38::*;

/// Terra's Server type
pub struct Server {
    url: &'static str,
    out: Sender,
}

/// Terra's circle geofence type
pub struct TerraCircle<'a> {
    fleet_name: &'a str,
    lat: &'a str,
    lng: &'a str,
    radius: &'a str,
}

/// Terra's polygonal geofence type
pub struct TerraPolygon<'a> {
    fleet_name: &'a str,
    coordinates: Vec<Vec<f64>>,
}

/// Terra server's behavior
impl Server {
    /// Start the server at the given address!
    pub fn start(url: &'static str) {
        listen(url, |out: Sender| Server { url, out }).unwrap()
    }
    /// circular fence
    fn circular_fence(fence: TerraCircle) {}
}

/// `Handler` trait implementation for Terra server!
impl Handler for Server {
    /// Called when the WebSocket handshake is successful and the connection has
    /// been established
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        if let Some(addr) = handshake.remote_addr()? {
            println!("Connection open with {}", addr);
        }
        Ok(())
    }

    /// Called when a close frame from client is received!
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closing due to ({:?}) {}", code, reason);
        self.out.close(code).unwrap();
    }

    /// Handle inbound messages here!
    fn on_message(&mut self, msg: Message) -> Result<()> {
        crossbeam::scope(|scope| {
            scope.spawn(|| {
                match msg.into_text() {
                    Ok(m) => println!("{}", m),
                    Err(e) => println!("ERR {:?}", e),
                }
            })
        });
        Ok(())
    }
}