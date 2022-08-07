use comrak::{format_commonmark, parse_document, Arena, ComrakOptions, ComrakRenderOptions};
use sqlformat::{format, FormatOptions, QueryParams};
use std::io::{self, BufRead};
use std::iter::repeat;
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

enum InputKind {
    Markdown,
    Sql,
}

impl FromStr for InputKind {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" => Ok(InputKind::Markdown),
            "sql" => Ok(InputKind::Sql),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid argument {}", s),
            )),
        }
    }
}

// CLI arguments
#[derive(StructOpt)]
struct Options {
    /// Type of input
    #[structopt(long, short = "k")]
    kind: Option<InputKind>,
    /// Max line width
    #[structopt(long, short = "w", default_value = "80")]
    width: usize,
}

impl Options {
    const SQL_INTRO: &'static str = "Enter a SQL string to be reformatted";
    const MD_INTRO: &'static str = "\
        Enter a GitHub Flavored Markdown string to be reformatted.\n  \
          - See https://github.github.com/gfm/ for the GFM spec";

    /// Reformats a raw markdown string
    fn format_markdown(&self, input: String) -> String {
        // Formatting options. Max line width is 80 characters
        let options = ComrakOptions {
            render: ComrakRenderOptions {
                width: self.width,
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

        // Convert to String type and drop the blank extra line
        String::from_utf8(output).unwrap().trim().to_owned()
    }

    /// Reformats a row SQL string
    fn format_sql(&self, input: String) -> String {
        let params = QueryParams::default();
        let options = FormatOptions::default();
        format(&input, &params, options)
    }

    /// Prints a prompt for the user and then collects their multiline input into a single String.
    /// The input is only terminated when consecutive empty lines are enterred.
    fn get_input(&self) -> String {
        self.print_introduction();

        let stdin = io::stdin();
        let mut raw_lines = stdin.lock().lines();
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

    pub fn make_pretty(&self) {
        let input = self.get_input();
        let output = match self.kind {
            Some(InputKind::Markdown) | None => self.format_markdown(input),
            Some(InputKind::Sql) => self.format_sql(input),
        };

        let delimeter: String = repeat('-').take(self.width).collect();
        println!("Formatted output:\n{}\n{}\n{}", delimeter, output, delimeter);
    }

    fn print_introduction(&self) {
        let intro = match self.kind {
            Some(InputKind::Markdown) | None => Self::MD_INTRO,
            Some(InputKind::Sql) => Self::SQL_INTRO,
        };

        println!("\n{}", intro);
        println!("  - The string must not contain two (or more) consecutive newlines");
        println!("  - Press the Return key thrice to indicate when the input has terminated.\n");
    }
}

fn main() {
    let options = Options::from_args();
    options.make_pretty();
}
