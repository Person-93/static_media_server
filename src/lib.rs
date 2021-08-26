mod responders;
mod templating;

use actix_web::http::header;
use actix_web::web::{self, ServiceConfig};
use actix_web::{middleware, HttpResponse};
use responders::Dispatcher;
use std::path::PathBuf;

struct State {
    prefix_path: PathBuf,
}

pub fn configurator(
    prefix_path: PathBuf,
) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    templating::init();
    Box::from(|service_config: &mut ServiceConfig| {
        service_config.data(State { prefix_path }).service(
            web::scope("/")
                .wrap(middleware::DefaultHeaders::new().header(
                    header::CONTENT_TYPE,
                    mime::TEXT_HTML_UTF_8.to_string(),
                ))
                .service(web::resource("**").to(run_dispatcher)),
        );
    })
}

async fn run_dispatcher(
    req: web::HttpRequest,
    state: web::Data<State>,
) -> HttpResponse {
    let path = req.path();
    let path = match urlencoding::decode(path) {
        Ok(path) => PathBuf::from(path.into_owned()),
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string())
        }
    };
    responders::run_responder(Dispatcher::new(&path, &state.prefix_path)).await
}
