use super::directory::DirectoryResponder;
use super::not_found::NotFoundResponder;
use super::unimplemented::UnimplementedResponder;
use super::PathWithPrefix;
use super::{Responder, ResponderError, ResponderResult};
use actix_web::HttpResponse;
use async_trait::async_trait;
use std::error::Error;
use std::path::Path;

pub struct Dispatcher<'a> {
    path: &'a Path,
    prefix_path: &'a Path,
}

impl<'a> Dispatcher<'a> {
    pub fn new(path: &'a Path, prefix_path: &'a Path) -> Dispatcher<'a> {
        Dispatcher { path, prefix_path }
    }

    async fn convert_error(
        &self,
        err: &(dyn Error + 'static),
    ) -> ResponderResult<HttpResponse> {
        let prefixed_path = self.prefixed_path();
        match err.downcast_ref::<std::io::Error>() {
            Some(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                let responder =
                    NotFoundResponder::new(self.path, &prefixed_path);
                return Ok(super::run_responder(responder).await);
            }
            Some(io_err) => {
                return Ok(HttpResponse::InternalServerError()
                    .body(format!("Unknown IO Error: {}", io_err)));
            }
            None => (),
        }

        Ok(HttpResponse::InternalServerError()
            .body(format!("Unknown error: {}", err)))
    }
}

impl<'a> PathWithPrefix for Dispatcher<'a> {
    fn get_path(&self) -> &Path {
        self.path
    }

    fn get_path_prefix(&self) -> &Path {
        self.prefix_path
    }
}

#[async_trait(?Send)]
impl<'a> Responder for Dispatcher<'a> {
    async fn respond(&self) -> ResponderResult<HttpResponse> {
        let prefixed_path = self.prefixed_path();
        // TODO canonicalize the path on startup?
        let prefixed_path = match prefixed_path.canonicalize() {
            Ok(path_prefix) => path_prefix,
            Err(e) => {
                return Ok(if e.kind() == std::io::ErrorKind::NotFound {
                    super::run_responder(NotFoundResponder::new(
                        self.path,
                        &prefixed_path,
                    ))
                    .await
                } else {
                    generic_error_response(&e)
                })
            }
        };

        if prefixed_path.is_dir() {
            return Ok(super::run_responder(DirectoryResponder::new(
                self.path,
                self.prefix_path,
            ))
            .await);
        };

        Ok(super::run_responder(UnimplementedResponder::new(self.path)).await)
    }

    #[allow(clippy::async_yields_async)]
    async fn error_map(&self, err: ResponderError) -> HttpResponse {
        match err.source() {
            Some(parent) => match self.convert_error(parent).await {
                Ok(response) => response,
                Err(err) => generic_error_response(&err),
            },
            None => generic_error_response(&err),
        }
    }
}

fn generic_error_response(err: &dyn Error) -> HttpResponse {
    HttpResponse::InternalServerError()
        .body(format!("Error dispatching request: {}", err))
}
