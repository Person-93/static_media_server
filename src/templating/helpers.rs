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

    let file = helper
        .param(1)
        .ok_or_else(|| {
            RenderError::new(
            "Second param to `file_entry` must be the file name as a string",
        )
        })?
        .value()
        .as_str()
        .ok_or_else(|| {
            RenderError::new("File name is not a valid UTF-8 string")
        })?;

    let path = path.iter().last().map(|v| v.to_string());
    let path = match path {
        Some(parent) => {
            // remove quotes added by Value::tostring()
            let mut parent = parent.chars();
            parent.next();
            parent.next_back();
            let parent = parent.as_str();

            format!("{}/{}", parent, file)
        }
        None => file.to_owned(),
    };

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
