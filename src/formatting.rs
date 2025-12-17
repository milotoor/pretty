use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions, ComrakRenderOptions};
use sqlformat::{format, FormatOptions, QueryParams};
use std::process;

use crate::{
    links::render_links,
    options::{InputKind, Options},
};

/// Recursively strips bold (Strong) and italic (Emph) formatting from the AST,
/// replacing them with their plain text children.
fn strip_emphasis<'a>(node: &'a AstNode<'a>) {
    // First, recursively process all children
    for child in node.children() {
        strip_emphasis(child);
    }

    // Check if this node is Strong or Emph
    let should_unwrap = {
        let ast = node.data.borrow();
        matches!(ast.value, NodeValue::Strong | NodeValue::Emph)
    };

    if should_unwrap {
        // Move all children to be siblings before this node
        while let Some(child) = node.first_child() {
            child.detach();
            node.insert_before(child);
        }
        // Remove this now-empty emphasis node
        node.detach();
    }
}

/// Reformats a raw markdown string
fn format_markdown(input: impl Into<String>, width: usize, keep_formatting: bool) -> String {
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

    // Strip bold and italic formatting unless --keep-formatting is set
    if !keep_formatting {
        strip_emphasis(root);
    }

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
            let markdown = format_markdown(input, options.width, options.keep_emphasis);
            render_links(markdown, &links)
        }
        InputKind::Sql => format_sql(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_markdown_basic() {
        let input = "# Heading

This is a paragraph.";
        let expected_output = "# Heading

This is a paragraph.";
        assert_eq!(format_markdown(input, 80, true), expected_output);
    }

    #[test]
    fn test_format_markdown_wrapping() {
        let input = "# Heading

This is a paragraph with a lot of text that should wrap to a new line based on the specified width.";
        let expected_output = "# Heading

This is a paragraph with a lot of text that should wrap to a
new line based on the specified width.";
        assert_eq!(format_markdown(input, 60, true), expected_output);
    }

    #[test]
    fn test_format_markdown_with_lists() {
        let input = "1. Item one
2. Item two";
        let expected_output = "1.  Item one
2.  Item two";
        assert_eq!(format_markdown(input, 80, true), expected_output);
    }

    #[test]
    fn test_format_markdown_with_code_block() {
        let input = "Here is some code:

```
let x = 10;
println!(\"x is {}\", x);
```";
        let expected_output = "Here is some code:

    let x = 10;
    println!(\"x is {}\", x);";
        assert_eq!(format_markdown(input, 80, true), expected_output);
    }

    #[test]
    fn test_format_markdown_empty_input() {
        let input = "";
        let expected_output = "";
        assert_eq!(format_markdown(input, 80, true), expected_output);
    }

    #[test]
    fn test_format_sql_basic() {
        let input = "SELECT * FROM users WHERE id = 1;";
        let expected_output = "SELECT
  *
FROM
  users
WHERE
  id = 1;";
        assert_eq!(format_sql(input.to_string()), expected_output);
    }

    #[test]
    fn test_format_sql_with_multiple_queries() {
        let input = "SELECT * FROM users; INSERT INTO users (id, name) VALUES (1, 'Alice');";
        let expected_output = "SELECT
  *
FROM
  users;
INSERT INTO
  users (id, name)
VALUES
  (1, 'Alice');";
        assert_eq!(format_sql(input.to_string()), expected_output);
    }

    #[test]
    fn test_format_sql_with_complex_query() {
        let input = "SELECT u.id, u.name, p.total FROM users u JOIN purchases p ON u.id = p.user_id WHERE p.total > 100 ORDER BY p.total DESC;";
        let expected_output = "SELECT
  u.id,
  u.name,
  p.total
FROM
  users u
  JOIN purchases p ON u.id = p.user_id
WHERE
  p.total > 100
ORDER BY
  p.total DESC;";
        assert_eq!(format_sql(input.to_string()), expected_output);
    }

    #[test]
    fn test_format_sql_empty_input() {
        let input = "";
        let expected_output = "";
        assert_eq!(format_sql(input.to_string()), expected_output);
    }

    #[test]
    fn test_link_escaping() {
        let input = "Check out this link[1].";
        let expected_output = "Check out this link[1].";
        let formatted = format_markdown(input, 80, true);
        assert_eq!(formatted, expected_output);
    }

    #[test]
    fn test_strip_bold_formatting() {
        let input = "This is **bold** text.";
        let expected_output = "This is bold text.";
        assert_eq!(format_markdown(input, 80, false), expected_output);
    }

    #[test]
    fn test_strip_italic_formatting() {
        let input = "This is *italic* text.";
        let expected_output = "This is italic text.";
        assert_eq!(format_markdown(input, 80, false), expected_output);
    }

    #[test]
    fn test_strip_bold_and_italic_formatting() {
        let input = "This has **bold** and *italic* and ***both***.";
        let expected_output = "This has bold and italic and both.";
        assert_eq!(format_markdown(input, 80, false), expected_output);
    }

    #[test]
    fn test_keep_emphasis_preserves_bold_and_italic() {
        let input = "This is **bold** and *italic* text.";
        let expected_output = "This is **bold** and *italic* text.";
        assert_eq!(format_markdown(input, 80, true), expected_output);
    }

    #[test]
    fn test_strip_nested_emphasis() {
        let input = "This is **bold with *nested italic* inside**.";
        let expected_output = "This is bold with nested italic inside.";
        assert_eq!(format_markdown(input, 80, false), expected_output);
    }
}
