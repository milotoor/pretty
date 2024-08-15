use comrak::{format_commonmark, parse_document, Arena, ComrakOptions, ComrakRenderOptions};
use sqlformat::{format, FormatOptions, QueryParams};
use std::process;

use crate::{
    links::render_links,
    options::{InputKind, Options},
};

/// Reformats a raw markdown string
fn format_markdown(input: impl Into<String>, width: usize) -> String {
    // Formatting options. Max line width is 80 characters
    let options = ComrakOptions {
        render: ComrakRenderOptions {
            width,
            ..ComrakRenderOptions::default()
        },
        ..ComrakOptions::default()
    };

    // Parse the document. The returned nodes are created in the supplied Arena, and are bound by
    // its lifetime. An Arena is basically a fast mechanism for allocating numerous values of the
    // same type.
    let arena = Arena::new();
    let root = parse_document(&arena, &input.into(), &options);

    // Reformat the AST into a vector of UTF-8 charcodes
    let mut output = vec![];
    format_commonmark(root, &options, &mut output).unwrap_or_else(|err| {
        eprintln!("Problem reformatting input: {}", err);
        process::exit(1);
    });

    // Convert to String type and drop the blank extra line and replace escaped brackets with
    // actual brackets
    String::from_utf8(output)
        .unwrap()
        .trim()
        .to_owned()
        .replace(r"\[", "[")
        .replace(r"\]", "]")
}

/// Reformats a raw SQL string
fn format_sql(input: String) -> String {
    let params = QueryParams::default();
    let options = FormatOptions::default();
    format(&input, &params, options)
}

pub fn format_output(options: &Options, input: String, links: Vec<String>) -> String {
    match options.kind {
        InputKind::Markdown => {
            let markdown = format_markdown(input, options.width);
            render_links(markdown, &links)
        }
        InputKind::Sql => format_sql(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_escaping() {
        let input = "Check out this link[1].";
        let expected_output = "Check out this link[1].";
        let formatted = format_markdown(input, 80);
        assert_eq!(formatted, expected_output);
    }
}
