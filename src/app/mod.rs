mod main;

use {ContextState, HttpHandler, Rc};

/// App loader
pub fn load_apps() -> Vec<Box<HttpHandler>> {
    //Compile the tera templates
    let tera = Rc::new(compile_templates!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/templates/**/*"
    )));
    //Context wrapper
    let context = Rc::new(ContextState { tera: tera.clone() });

    // Register app here
    // App need to be boxed
    vec![main::app(context.clone())]
}
