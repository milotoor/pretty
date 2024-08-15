use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::io::{self, BufRead};
use std::process;

use crate::links::extract_links;
use crate::options::InputKind;
use crate::options::InputSource;
use crate::options::Options;

/// Gets the contents of the clipboard
fn get_input_from_clipboard() -> String {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.get_contents().unwrap_or_else(|err| {
        eprintln!("Problem getting clipboard contents: {}", err);
        process::exit(1);
    })
}

/// Prints a prompt for the user and then collects their multiline input into a single String.
/// The input is only terminated when consecutive empty lines are enterred.
fn get_input_from_stdin(options: &Options) -> String {
    print_introduction(options);

    let mut raw_lines = io::stdin().lock().lines();
    let mut parsed_lines: Vec<String> = vec![];

    while let Some(raw_line) = raw_lines.next() {
        // Trim whitespace from the input line, exiting if we couldn't parse it for some reason
        let line: String = raw_line.unwrap_or_else(|err| {
            eprintln!("Problem parsing input: {}", err);
            process::exit(1);
        });

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

const SQL_INTRO: &'static str = "Enter a SQL string to be reformatted";
const MD_INTRO: &'static str = "\
      Enter a GitHub Flavored Markdown string to be reformatted.\n  \
        - See https://github.github.com/gfm/ for the GFM spec";

fn print_introduction(options: &Options) {
    let intro = match options.kind {
        InputKind::Markdown => MD_INTRO,
        InputKind::Sql => SQL_INTRO,
    };

    println!("\n{}", intro);
    println!("  - The string must not contain two (or more) consecutive newlines");
    println!("  - Press the Return key thrice to indicate when the input has terminated.\n");
}

pub fn get_input(options: &Options) -> (String, Vec<String>) {
    extract_links(match options.input_src {
        InputSource::Stdin => get_input_from_stdin(&options),
        InputSource::Clipboard => get_input_from_clipboard(),
    })
}
