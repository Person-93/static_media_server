use super::super::templating::{self, PathOnly, Templates};
use super::Responder;
use super::{ResponderError, ResponderResult};
use actix_web::HttpResponse;
use async_trait::async_trait;
use log::info;
use std::path::Path;

pub struct NotFoundResponder<'a> {
    path: &'a Path,
    full_path: &'a Path,
}

impl<'a> NotFoundResponder<'a> {
    pub fn new(path: &'a Path, full_path: &'a Path) -> NotFoundResponder<'a> {
        NotFoundResponder { path, full_path }
    }
}

#[async_trait(?Send)]
impl<'a> Responder for NotFoundResponder<'a> {
    async fn respond(&self) -> ResponderResult<HttpResponse> {
        info!("File not found: {}", self.full_path.to_string_lossy());
        let rendered = templating::render(Templates::NotFound(
            PathOnly::new(self.path).map_err(|message| {
                ResponderError::new(RESPONDER_NAME, message)
            })?,
        ))
        .map_err(|e| {
            ResponderError::new(RESPONDER_NAME, "render error")
                .with_error(Box::new(e))
        })?;

        Ok(HttpResponse::NotFound().body(rendered))
    }
}

const RESPONDER_NAME: &str = "not_found";
