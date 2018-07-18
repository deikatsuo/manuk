extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

mod app;
mod state;

use actix_web::{
    error, fs, http, middleware::Logger, server::{self, HttpHandler}, App, Error, HttpRequest,
    HttpResponse, Responder, State,
};
use state::{AppState, ContextState};
use std::rc::Rc;
use tera::Tera;

fn main() {
    // Create a Logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("manuk");

    // Create a new Server
    server::new(|| {
        // Load the modules
        app::load_apps()
    }).bind("127.0.0.1:8000")
        .expect("Failed to bind the address...")
        .start();

    println!("Running...");
    sys.run();
}
