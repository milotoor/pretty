use clipboard::{ClipboardContext, ClipboardProvider};
use std::iter::repeat;

use crate::options::{Options, OutputDestination};

pub fn write_file(options: &Options, output: String) {
    match options.output_dest {
        OutputDestination::Stdout => {
            let delimeter: String = repeat('-').take(options.width).collect();
            println!(
                "Formatted output:\n{}\n{}\n{}",
                delimeter, output, delimeter
            );
        }

        OutputDestination::Clipboard => {
            // Copy the output to the clipboard
            let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
            clipboard.set_contents(output).unwrap();
            println!("✅ Output copied to clipboard");
        }
    }
}
