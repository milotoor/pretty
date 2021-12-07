use comrak::{format_commonmark, parse_document, Arena, ComrakOptions, ComrakRenderOptions};
use std::io::{self, BufRead};
use std::process;

fn main() {
    let input = get_input();
    let output = format_input(input);
    println!(
        "Formatted output:\n\
        -----------------\n\n{}",
        output
    );
}

/// Prints a prompt for the user and then collects their multiline input into a single String. The
/// input is only terminated when consecutive empty lines are enterred.
fn get_input() -> String {
    println!(
        "Enter a GitHub Flavored Markdown string to be reformatted.\n\
          - The string must not contain two (or more) consecutive newlines\n\
          - See https://github.github.com/gfm/ for the GFM spec \n\
          - Press the Return key twice to indicate when the input has terminated.\n"
    );

    let stdin = io::stdin();
    let mut raw_lines = stdin.lock().lines();
    let mut parsed_lines: Vec<String> = vec![];

    while let Some(raw_line) = raw_lines.next() {
        // Trim whitespace from the input line, exiting if we couldn't parse it for some reason
        let line: String = raw_line
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing input: {}", err);
                process::exit(1);
            })
            .trim()
            .into();

        // If the line is empty, check if the last line was too. If so, break out of the loop (and
        // remove the previous line which we know to be blank)
        if line == "" && parsed_lines.last() == Some(&String::from("")) {
            parsed_lines.pop();
            break;
        }

        // Line is non-empty, so append it to our vector
        parsed_lines.push(line);
    }

    // Fold the vector of strings into one concatenated string separated by newline characters
    parsed_lines.join("\n")
}

/// Takes the raw markdown input from the user and reformats it
fn format_input(input: String) -> String {
    // Formatting options. Max line width is 80 characters
    let options = ComrakOptions {
        render: ComrakRenderOptions {
            width: 80,
            ..ComrakRenderOptions::default()
        },
        ..ComrakOptions::default()
    };

    // Parse the document. The returned nodes are created in the supplied Arena, and are bound by
    // its lifetime. An Arena is basically a fast mechanism for allocating numerous values of the
    // same type.
    let arena = Arena::new();
    let root = parse_document(&arena, &input, &options);

    // Reformat the AST into a vector of UTF-8 charcodes
    let mut output = vec![];
    format_commonmark(root, &options, &mut output).unwrap_or_else(|err| {
        eprintln!("Problem reformatting input: {}", err);
        process::exit(1);
    });

    // Convert to String type
    String::from_utf8(output).unwrap()
}
