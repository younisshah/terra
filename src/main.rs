extern crate clap;
extern crate terra;

use terra::geofence::Server;
use clap::{App, Arg};

fn main() {
    init();
}

/// Terra init
fn init() {
    let banner = r#"
              ________
              ___  __/_____ ______________________ _
              __  /   _  _ \__  ___/__  ___/_  __ `/       Terra is a Tile38 based Geofence server.
              _  /    /  __/_  /    _  /    / /_/ /        v0.0.1
              /_/     \___/ /_/     /_/     \__,_/"#;

    let app = App::new("Terra")
        .version("0.0.1")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .help("Set the host address to listen for new connections")
        )
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Set the port number to listen for new connections")
        )
        .get_matches();

    println!("{}", banner);
    let server_addr = format!("{}:{}", app.value_of("host").unwrap_or("127.0.0.1"), app.value_of("port").unwrap_or("9761"));
    println!("\nTerra started! Listening for incoming connections on `{}`.", server_addr);
    Server::start(server_addr)
}