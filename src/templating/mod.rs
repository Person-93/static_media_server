mod helpers;

use handlebars::{Handlebars, RenderError};
use lazy_static::lazy_static;
use serde::Serialize;
use std::path::Path;

pub fn init() {
    lazy_static::initialize(&HANDLEBARS);
}

pub fn render(template: Templates) -> Result<String, RenderError> {
    match template {
        Templates::Unimplemented(info) => {
            HANDLEBARS.render(TEMPLATE_UNIMPLEMENTED, &info)
        }
        Templates::Directory(info) => {
            HANDLEBARS.render(TEMPLATE_DIRECTORY, &info)
        }
        Templates::NotFound(info) => {
            HANDLEBARS.render(TEMPLATE_NOT_FOUND, &info)
        }
    }
}

pub enum Templates<'a> {
    Unimplemented(PathOnly<'a>),
    Directory(Directory<'a>),
    NotFound(PathOnly<'a>),
}

type StrAsError<T = ()> = Result<T, &'static str>;

#[derive(Serialize)]
pub struct Directory<'a> {
    /// The path with the serve point as the root
    path: Vec<&'a str>,
    /// A list of the directory's contents
    files: Vec<&'a str>,
}

impl<'a> Directory<'a> {
    pub fn new(
        path: &'a Path,
        files: Vec<&'a str>,
    ) -> StrAsError<Directory<'a>> {
        let path = path_to_slice(path)?;
        Ok(Directory { path, files })
    }
}

#[derive(Serialize)]
pub struct PathOnly<'a> {
    path: Vec<&'a str>,
}

impl<'a> PathOnly<'a> {
    pub fn new(path: &'a Path) -> Result<PathOnly, &'static str> {
        let path = path_to_slice(path)?;
        Ok(PathOnly { path })
    }
}

fn path_to_slice(path: &Path) -> Result<Vec<&str>, &'static str> {
    path.iter()
        .map(|segment| -> Result<&str, &'static str> {
            segment.to_str().ok_or("Path must be a valid UTF-8 string")
        })
        .collect()
}

const TEMPLATE_DIRECTORY: &str = "directory";
const TEMPLATE_UNIMPLEMENTED: &str = "unimplemented";
const TEMPLATE_NOT_FOUND: &str = "not_found";

lazy_static! {
    static ref HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();

        helpers::register(&mut handlebars);

        handlebars
            .register_template_string(
                "base",
                include_str!("templates/base.hbs"),
            )
            .unwrap();
        handlebars
            .register_template_string(
                "file",
                include_str!("templates/file.hbs"),
            )
            .unwrap();
        handlebars
            .register_template_string(
                TEMPLATE_DIRECTORY,
                include_str!("templates/directory.hbs"),
            )
            .unwrap();
        handlebars
            .register_template_string(
                TEMPLATE_UNIMPLEMENTED,
                include_str!("templates/unimplemented.hbs"),
            )
            .unwrap();
        handlebars
            .register_template_string(
                TEMPLATE_NOT_FOUND,
                include_str!("templates/not_found.hbs"),
            )
            .unwrap();

        handlebars
    };
}
