use super::super::templating;
use super::{Responder, ResponderError, ResponderResult};
use crate::templating::{PathOnly, Templates};
use actix_web::HttpResponse;
use async_trait::async_trait;
use std::path::Path;

pub struct UnimplementedResponder<'a> {
    path: &'a Path,
}

impl<'a> UnimplementedResponder<'a> {
    pub fn new(path: &'a Path) -> UnimplementedResponder<'a> {
        UnimplementedResponder { path }
    }
}

#[async_trait(?Send)]
impl<'a> Responder for UnimplementedResponder<'a> {
    async fn respond(&self) -> ResponderResult<HttpResponse> {
        let rendered = templating::render(Templates::Unimplemented(
            PathOnly::new(self.path).map_err(|message| {
                ResponderError::new(RESPONDER_NAME, message)
            })?,
        ))
        .map_err(|e| {
            ResponderError::new(RESPONDER_NAME, "render error")
                .with_error(Box::new(e))
        })?;

        Ok(HttpResponse::NotImplemented().body(rendered))
    }
}

const RESPONDER_NAME: &str = "unimplemented";
