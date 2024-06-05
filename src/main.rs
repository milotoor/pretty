use clap::Parser;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions, ComrakRenderOptions};
use sqlformat::{format, FormatOptions, QueryParams};
use std::io::{self, BufRead};
use std::iter::repeat;
use std::process;

#[derive(clap::ValueEnum, Debug, Clone)]
enum InputKind {
    Markdown,
    Sql,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum InputSource {
    Stdin,
    Clipboard,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum OutputDestination {
    Stdout,
    Clipboard,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Options {
    /// The input source
    #[arg(short, long, default_value = "stdin")]
    input_src: InputSource,

    /// The output destination
    #[arg(short, long, default_value = "clipboard")]
    output_dest: OutputDestination,

    /// Type of input
    #[arg(short, long, default_value = "markdown")]
    kind: InputKind,

    /// Max line width
    #[arg(short, long, default_value = "80")]
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

    /// Gets the contents of the clipboard
    fn get_input_from_clipboard(&self) -> String {
        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        clipboard.get_contents().unwrap_or_else(|err| {
            eprintln!("Problem getting clipboard contents: {}", err);
            process::exit(1);
        })
    }

    /// Prints a prompt for the user and then collects their multiline input into a single String.
    /// The input is only terminated when consecutive empty lines are enterred.
    fn get_input_from_stdin(&self) -> String {
        self.print_introduction();

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

    pub fn make_pretty(&self) {
        let input = match self.input_src {
            InputSource::Stdin => self.get_input_from_stdin(),
            InputSource::Clipboard => self.get_input_from_clipboard(),
        };

        let output = match self.kind {
            InputKind::Markdown => self.format_markdown(input),
            InputKind::Sql => self.format_sql(input),
        };

        match self.output_dest {
            OutputDestination::Stdout => {
              let delimeter: String = repeat('-').take(self.width).collect();
              println!("Formatted output:\n{}\n{}\n{}", delimeter, output, delimeter);
            },

            OutputDestination::Clipboard => {
              // Copy the output to the clipboard
              let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
              clipboard.set_contents(output).unwrap();
              println!("✅ Output copied to clipboard");
            },
        }
    }

    fn print_introduction(&self) {
        let intro = match self.kind {
            InputKind::Markdown => Self::MD_INTRO,
            InputKind::Sql => Self::SQL_INTRO,
        };

        println!("\n{}", intro);
        println!("  - The string must not contain two (or more) consecutive newlines");
        println!("  - Press the Return key thrice to indicate when the input has terminated.\n");
    }
}

fn main() {
    let options = Options::parse();
    options.make_pretty();
}
