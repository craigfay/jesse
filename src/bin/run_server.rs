
extern crate gotham;
extern crate rust_json_api;

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;

use rust_json_api::db;
use serde_json;


pub fn read_post_handler(state: State) -> (State, String) {
    let post = db::read_post(1);
    let json = serde_json::to_string_pretty(&post).unwrap();
    (state, json.to_string())
}

pub fn read_posts_handler(state: State) -> (State, String) {
    let posts = db::read_posts();
    let json = serde_json::to_string_pretty(&posts).unwrap();
    (state, json.to_string())
}

fn router() -> Router {
    build_simple_router(|route| {
        route.scope("/api", |route| {
            route
                .get("/posts.json")
                .to(read_posts_handler);

            route
                .get("/posts/:id.json")
                .to(read_post_handler);
        });
    })
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}


