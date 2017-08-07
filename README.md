## Terra

[Tile38](http://tile38.com) is an open source (MIT licensed), in-memory geolocation data store, spatial index,
and realtime geofence. It supports a variety of object types including lat/lon points, bounding boxes, XYZ tiles,
Geohashes, and GeoJSON.

Terra is a Tile38 based Geofence server in Rust. Terra is meant to be a memory-safe
and highly concurrent geofence server.

Terra is meant to be used for my personal location-based projects. It in no way
is a replacement for the amazing Tile38 server, but is actually an augmentation to it.

Terra is **NOT** available as a __crate__ on [crates.io](http://crates.io) yet, although I plan to.

Terra supports two types of Geofences: **Circular** and **Polygonal**. (so far)

#### Getting started

The easiest way to to clone the project and run as a Rust binary project like this:

`~$ git clone https://github.com/younisshah/terra.git`

`~$ cd terra`

`~$ cargo check`

`~$ cargo build`

`~$ cargo run`

If all went well, you'll see something like this:

   ________
              ___  __/_____ ______________________ _
              __  /   _  _ \__  ___/__  ___/_  __ `/       Terra is a Tile38 based Geofence server.
              _  /    /  __/_  /    _  /    / /_/ /        v0.0.1
              /_/     \___/ /_/     /_/     \__,_/


[+] Checking if Tile38 is running at '127.0.0.1:9851' is live... <br/>
[+] Tile38 is RUNNING.<br/>
[+] Terra started! Listening for incoming connections on '127.0.0.1:9761'.<br/>

By default, Terra runs on port `9761` and expects an instance of Tile38 server running on it's default port `9851`.
However, the defaults can be changed. Run the following command to see Terra's usage:

`~$ ./target/debug/terra --help`

```ini
Terra 0.0.1 <br/>

 USAGE:<br/>
     terra [FLAGS]<br/>

 FLAGS:<br/>
         --help        Prints help information<br/>
     -h, --host        Set the host address to listen for new connections<br/>
     -p, --port        Set the port number to listen for new connections<br/>
         --t38_host    Set the host address of Tile38 server<br/>
         --t38_port    Set the port number of Tile38 server<br/>
     -V, --version     Prints version information<br/>
```


Once, Terra is up and running, create a WebSocket connection like this.

This is a JS client. You can use any WebSocket client:

```javascript
const socket = new WebSocket("ws://localhost:9761");
socket.onmessage = function(event) {
  console.log(event.data);
};
```

Send fence creation request:


##### To create a circular geofence:

Send the following data:

```javascript
const fence = {fence_type: "circle", fleet_name: "my_circular_fence", lat: "12.3", lng: "34.4", radius: "6000"};
socket.send(fence)
```


##### To create a polygonal geofence:

Send the following data:

```javascript
const fence = { fence_type: "polygon", fleet_name : "my_polygonal_fence", id: "some_id", coordinates: [[12.12, 43.32],[12.12, 53.32],[12.4, 55.2],[12.12, 43.32]]};
socket.send(fence);
```

That's it!

#### TODO

- [ ] Write sane documentation.
- [ ] Expose server API.
- [ ] Change to a crate.
- [ ] Add more shapes of geofences.