/**
A simple demonstration of an HTTP server
 
References:
http://ironframework.io/
*/

extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_:&mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello World")))
	}
	
	Iron::new(hello_world).http("localhost:3100").unwrap();
}
