use std::net::SocketAddr;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let params = Params::get();
    start(params).await
}

struct Params {
    socket: SocketAddr,
    path: PathBuf,
}

impl Params {
    fn get() -> Params {
        use clap::App;
        use clap::{load_yaml, ErrorKind};
        use std::env;

        let yaml = load_yaml!("cli.yml");
        let app = App::from(yaml)
            .name(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"));

        let matches = app.get_matches();

        let socket = matches.value_of_t("ip").unwrap();
        let path = match matches.value_of_t("dir") {
            Ok(path) => path,
            Err(err) => {
                if err.kind == ErrorKind::ArgumentNotFound {
                    env::current_dir().unwrap()
                } else {
                    err.exit()
                }
            }
        };

        Params { socket, path }
    }
}

async fn start(Params { socket, path }: Params) -> std::io::Result<()> {
    use actix_web::middleware::Logger;
    use actix_web::{App, HttpServer};
    use static_media_server::configurator;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(configurator(path.clone()))
    })
    .bind(socket)?
    .run()
    .await
}
