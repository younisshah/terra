extern crate ws;
extern crate nazar;
extern crate serde_json;

use self::ws::{listen, CloseCode, Sender, Handler, Handshake, Message, Result};
use self::nazar::t38::*;
use std::borrow::Cow;
use std::net::SocketAddr;
use serde_json::Value;

enum FenceType {
    Circle,
    Polygon,
    Unknown
}

/// Terra's Server type
pub struct Server {
    url: SocketAddr,
    out: Sender,
}

/// Terra's circle geofence type
#[derive(Debug, Deserialize)]
pub struct TerraCircle<'t> {
    fleet_name: &'t str,
    lat: &'t str,
    lng: &'t str,
    radius: &'t str,
}

/// Terra's polygonal geofence type
#[derive(Debug, Deserialize)]
pub struct TerraPolygon<'t> {
    fleet_name: &'t str,
    coordinates: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
struct _FenceType<'a> {
    fence_type: &'a str
}

/// Terra server's behavior
impl Server {
    /// Start Terra server!
    ///
    /// The URL of Terra server
    ///
    /// # Panics
    ///
    /// The `start` function panics if url is an empty string literal!
    pub fn start<'t, U>(url: U)
        where U: Into<Cow<'t, str>>
    {
        let cow_url = url.into();
        assert!(cow_url != "");
        let url: SocketAddr = cow_url.parse().unwrap();
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
        match msg.into_text() {
            Ok(m) => {
                use self::FenceType::{Circle, Polygon, Unknown};
                let json = &m[..];
                let fence_type = get_type(json);
                match fence_type {
                    Circle => {
                        let c: TerraCircle = serde_json::from_str(json).unwrap_or(get_empty_circular_fence());
                        println!("c= {:?}", c);
                    },
                    Polygon => {
                        let p: TerraPolygon = serde_json::from_str(json).unwrap_or(get_empty_poly_fence());
                        println!("p = {:?}", p);
                    },
                    Unknown => {
                        println!("UNKNOWN");
                    }
                }
                Ok(())
            }
            Err(e) => {
                println!("ERR {:?}", e);
                Err(e)
            }
        }
    }
}

/// parse incoming JSON to get the type of Geofence
fn get_type<'a>(json: &'a str) -> FenceType {
    let v: _FenceType = serde_json::from_str(json).unwrap_or(_FenceType { fence_type: "unknown" });
    match v.fence_type {
        "circle" => FenceType::Circle,
        "polygon" => FenceType::Polygon,
        _ => FenceType::Unknown,
    }
}

fn get_empty_circular_fence<'a>() -> TerraCircle<'a> {
    TerraCircle { fleet_name: "", lat: "", lng: "", radius: "" }
}

fn get_empty_poly_fence<'a>() -> TerraPolygon<'a> {
    TerraPolygon { fleet_name: "", coordinates: vec![vec![]]}
}