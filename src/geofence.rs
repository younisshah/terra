extern crate ws;
extern crate nazar;
extern crate serde_json;

use self::ws::{listen, CloseCode, Sender, Handler, Handshake, Message, Result};
use self::nazar::t38::{NazarSender};
use std::cmp::Ordering;
use std::borrow::Cow;

enum FenceType {
    Circle,
    Polygon,
    Unknown
}

/// Terra's Server type
pub struct Server {
    t38_url: String,
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
    id: &'t str,
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
    /// The `start` function panics if either of `url` or `t38_url` is an empty Cow string!
    pub fn start<'t, U>(url: U, t38_url: U)
        where U: Into<Cow<'t, str>>
    {
        let url = url.into().to_string();
        let t38_url = t38_url.into().to_string();

        assert_ne!(url.cmp(&"".to_string()), Ordering::Equal);
        assert_ne!(t38_url.cmp(&"".to_string()), Ordering::Equal);

        listen(url, |out: Sender| Server { t38_url: t38_url.clone(), out }).unwrap()
    }
}

/// `Handler` trait implementation for Terra server!
impl Handler for Server {
    /// Called when the WebSocket handshake is successful and the connection has
    /// been established
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        if let Some(addr) = handshake.remote_addr()? {
            println!("[+] Connected with {}", addr);
        }
        Ok(())
    }

    /// Called when a close frame from client is received!
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("[*] Closing connection due to ({:?}) {}", code, reason);
        //self.out.close(code).unwrap();
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
                        println!("[+] Opening a circular geofence = {:?}", c);
                        let n = nazar::t38::Client::new();
                        n.open_fence2(&format!("ws://{}", self.t38_url)[..], c.fleet_name, c.lat, c.lng, c.radius, action);
                    }
                    Polygon => {
                        let p: TerraPolygon = serde_json::from_str(json).unwrap_or(get_empty_poly_fence());
                        println!("[+] Opening a polygonal geofence = {:?}", p);
                        let n = nazar::t38::Client::new();
                        n.open_fence_within2(&format!("ws://{}", self.t38_url)[..], p.fleet_name, p.id, p.coordinates, action);
                    }
                    Unknown => {
                        println!("[*] Unknown geofence type. Valid types are 'circle' and 'polygon'.\n\
                        Please check the 'fence_type'.");
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

/// Send the fence updates on the connected WebSocket!
fn action(out: &NazarSender, msg: String) {
    println!("[+] Fence update: {}", msg);
    out.send(msg).unwrap_or_default();
}

fn get_empty_circular_fence<'a>() -> TerraCircle<'a> {
    TerraCircle { fleet_name: "", lat: "", lng: "", radius: "" }
}

fn get_empty_poly_fence<'a>() -> TerraPolygon<'a> {
    TerraPolygon { fleet_name: "", id: "", coordinates: vec![vec![]] }
}