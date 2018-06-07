extern crate env_logger;
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate tera;

mod state;
mod app;

use std::rc::Rc;
use actix_web::{
    error, fs, http, App, Error, HttpResponse, HttpRequest, Responder, State,
    server::{
        self, HttpHandler
    },
    middleware::{
        Logger
    }
};
use tera::Tera;
use state::{AppState, ContextState};

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