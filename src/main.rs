use clap::Parser;

mod formatting;
mod input;
mod links;
mod options;
mod write;

use formatting::format_output;
use input::get_input;
use options::Options;
use write::write_file;

fn make_pretty(options: &Options) {
    let (input, links) = get_input(options);
    let output = format_output(options, input, links);
    write_file(options, output);
}

fn main() {
    let options = Options::parse();
    make_pretty(&options);
}
