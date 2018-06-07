use super::{fs, http, error, Error, HttpRequest, HttpHandler, App,
    AppState, ContextState, Rc, Responder, Logger, State, Tera, HttpResponse};

fn index(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = super::super::tera::Context::new();
    ctx.add("name", &"Deri".to_owned());
    ctx.add("text", &"Welcome!".to_owned());
    let s = state
        .context.tera
        .render("index.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn app(context: Rc<ContextState>) -> Box<HttpHandler> {
    App::with_state(AppState { context })
        .prefix("/")
        .middleware(Logger::new("\nClient %a \nRequest \"%r\" \nStatus \"%s\" \n%{User-Agent}i"))
        .resource("", |r| r.method(http::Method::GET).with(index))
        .handler("/asset", fs::StaticFiles::new("./asset/"))
        .boxed()
}