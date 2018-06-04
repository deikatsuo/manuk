mod asset;
mod main;

use super::{fs, http, error, Error, App, AppState, ContextState, HttpRequest,
    HttpHandler, Responder, Rc, State, Logger, Tera, HttpResponse};

/// App loader
pub fn load_apps() -> Vec<Box<HttpHandler>> {
    //Compile the tera templates
    let tera = Rc::new(compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")));
    //Context wrapper
    let context = Rc::new(
        ContextState { tera: tera.clone() }
    );
    // Register app here
    vec![
        asset::app(),
        main::app(context.clone())
    ]
}