extern crate ruster;
use ruster::HelloWorldServer;

fn main() {
  HelloWorldServer.serve_forever();
}
