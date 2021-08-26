//! Helpers for the handlebars templates.

use handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError,
};
use serde_json::Value;
use std::path::PathBuf;

type RenderResult = Result<(), RenderError>;

pub fn register(handlebars: &mut Handlebars) {
    handlebars.register_helper("breadcrumbs", Box::from(breadcrumbs));
    handlebars.register_helper("file_entry", Box::from(file_entry));
}

fn file_entry(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> RenderResult {
    let path = &helper
        .param(0)
        .and_then(|item| item.value().as_array())
        .ok_or_else(|| {
            RenderError::new(
                "First param to `file_entry` must be the path as an array",
            )
        })?[1..];
    let mut path = values_as_str(path)
        .map_err(RenderError::new)?
        .iter()
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>();

    let file = helper
        .param(1)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| {
            RenderError::new(
            "Second param to `file_entry` must be the file name as a string",
        )
        })?;

    path.push(urlencoding::encode(file).into_owned());
    let path: PathBuf = path.iter().collect();
    let path = path.to_str().ok_or_else(|| {
        RenderError::new("Path in `file_entry` is not valid utf-8 string")
    })?;

    out.write(&format!("<a href=\"{}\">{}</a><br/>", path, file))?;

    Ok(())
}

fn breadcrumbs(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> RenderResult {
    let path = helper
        .param(0)
        .and_then(|v| v.value().as_array())
        .ok_or_else(|| {
            RenderError::new(
                "Param to `breadcrumbs` must be the path as an array",
            )
        })?;
    let path = values_as_str(path).map_err(RenderError::new)?;

    let (last_segment, path) = path.split_last().ok_or_else(|| {
        RenderError::new("Path in `breadcrumbs` must not be empty")
    })?;

    let mut current = PathBuf::new();
    for segment in path {
        current.push(segment);
        let current = current.to_str().unwrap();
        out.write(&format!(
            "<li class=\"crumb\"><a href=\"{}\">{}</a></li>",
            current, segment
        ))?;
    }

    out.write(&format!("<li class=\"crumb\">{}</a></li>", last_segment))?;

    Ok(())
}

fn values_as_str(values: &[Value]) -> Result<Vec<&str>, &'static str> {
    values
        .iter()
        .map(|item| -> Result<&str, &'static str> {
            item.as_str().ok_or("array values must be strings")
        })
        .collect()
}
