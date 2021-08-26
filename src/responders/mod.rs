mod directory;
mod dispatcher;
mod not_found;
mod unimplemented;

use actix_web::HttpResponse;
use async_trait::async_trait;
use log::info;
use std::error;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

pub use dispatcher::Dispatcher;

type ResponderResult<T = ()> = Result<T, ResponderError>;

pub async fn run_responder<T: Responder>(responder: T) -> HttpResponse {
    match responder.respond().await {
        Ok(response) => response,
        Err(err) => {
            info!("{:?}", err);
            responder.error_map(err).await
        }
    }
}

#[async_trait(?Send)]
pub trait Responder: Sync + Send {
    async fn respond(&self) -> ResponderResult<HttpResponse>;

    #[allow(clippy::async_yields_async)]
    async fn error_map(&self, err: ResponderError) -> HttpResponse {
        HttpResponse::InternalServerError()
            .body(format!("An error occurred\n{}", err))
    }
}

#[derive(Debug)]
pub struct ResponderError {
    responder: &'static str,
    message: &'static str,
    error: Option<Box<dyn error::Error + 'static>>,
}

impl ResponderError {
    pub fn new(
        responder: &'static str,
        message: &'static str,
    ) -> ResponderError {
        ResponderError {
            responder,
            message,
            error: None,
        }
    }

    pub fn with_error(self, error: Box<dyn Error + 'static>) -> ResponderError {
        ResponderError {
            error: Some(error),
            ..self
        }
    }
}

impl Display for ResponderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error {
            Some(e) => write!(
                f,
                "The {} responder raised an error: {}\nWith child: {}",
                self.responder, self.message, e
            ),
            None => write!(
                f,
                "The {} responder raised an error: {}",
                self.responder, self.message
            ),
        }
    }
}

impl Error for ResponderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.error {
            Some(parent) => Some(parent.as_ref()),
            None => None,
        }
    }

    fn description(&self) -> &str {
        self.message
    }
}

pub trait PathWithPrefix {
    fn get_path(&self) -> &Path;
    fn get_path_prefix(&self) -> &Path;
    fn prefixed_path(&self) -> PathBuf {
        let path = self.get_path();
        let path = path.strip_prefix("/").unwrap_or(path);
        self.get_path_prefix().join(path)
    }
}
