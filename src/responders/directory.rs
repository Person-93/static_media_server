use super::super::templating::{self, Directory, Templates};
use super::PathWithPrefix;
use super::ResponderError;
use super::{Responder, ResponderResult};
use actix_web::HttpResponse;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

pub struct DirectoryResponder<'a> {
    /// The path to the directory with the serve-point as the root
    path: &'a Path,
    /// The path to the serve-point
    path_prefix: &'a Path,
}

impl<'a> DirectoryResponder<'a> {
    pub fn new(
        path: &'a Path,
        path_prefix: &'a Path,
    ) -> DirectoryResponder<'a> {
        DirectoryResponder { path, path_prefix }
    }
}

impl<'a> PathWithPrefix for DirectoryResponder<'a> {
    fn get_path(&self) -> &Path {
        self.path
    }

    fn get_path_prefix(&self) -> &Path {
        self.path_prefix
    }
}

#[async_trait(?Send)]
impl<'a> Responder for DirectoryResponder<'a> {
    async fn respond(&self) -> ResponderResult<HttpResponse> {
        let path_with_glob = self.prefixed_path().join("*");
        let pattern = path_with_glob.to_str().ok_or_else(|| {
            ResponderError::new(RESPONDER_NAME, "Error glob string")
        })?;
        let files = glob::glob(pattern)
            .map_err(|_| {
                ResponderError::new(RESPONDER_NAME, "Error creating glob")
            })?
            .collect::<Result<Vec<PathBuf>, _>>()
            .map_err(|_| {
                ResponderError::new(
                    RESPONDER_NAME,
                    "Pattern error while getting files",
                )
            })?;
        let files = files
            .iter()
            .map(|path| -> Result<&str, &'static str> {
                path.file_name()
                    .ok_or("Invalid path")?
                    .to_str()
                    .ok_or("Path is invalid utf-8 string")
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|message| ResponderError::new(RESPONDER_NAME, message))?;

        let rendered = templating::render(Templates::Directory(
            Directory::new(self.path, files).map_err(|message| {
                ResponderError::new(RESPONDER_NAME, message)
            })?,
        ))
        .map_err(|e| {
            ResponderError::new(RESPONDER_NAME, "render error")
                .with_error(Box::new(e))
        })?;

        Ok(HttpResponse::Ok().body(rendered))
    }
}

const RESPONDER_NAME: &str = "directory";
