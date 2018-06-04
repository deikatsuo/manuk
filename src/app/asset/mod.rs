// Static assets loader

use super::{fs, Tera, HttpHandler, App, Logger};


pub fn app() -> Box<HttpHandler> {
    App::new()
        .prefix("/asset")
        .middleware(Logger::new("\nClient %a \nRequest \"%r\" \nStatus \"%s\" \n%{User-Agent}i"))
        .handler("", fs::StaticFiles::new("./asset/"))
        .boxed()
}