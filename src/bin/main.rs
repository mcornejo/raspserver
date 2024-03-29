extern crate iron;
extern crate rand;
extern crate raspserver;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use raspserver::messages::JsonTestResponse;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;



fn pick_response() -> String {
    let num = rand::thread_rng().gen_range(1,4);
    let response = match num {
        1 => "Hello World!",
        2 => "Did you see that ludicrous display last night?",
        3 => "Nice weather for ducks",
        _ => ""
    };

    response.to_string()
}



fn main() {



    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();
        
        // create the response
        let response = JsonTestResponse { response: pick_response() };
        println!("{:?}", &response);

        let out = serde_json::to_string(&response).unwrap();

        Ok(Response::with((content_type, status::Ok, out)))
    }).http("localhost:3000").unwrap();
}