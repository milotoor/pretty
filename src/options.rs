use clap::Parser;

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum InputKind {
    Markdown,
    Sql,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum InputSource {
    Stdin,
    Clipboard,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum OutputDestination {
    Stdout,
    Clipboard,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// The input source
    #[arg(short, long, default_value = "stdin")]
    pub input_src: InputSource,

    /// The output destination
    #[arg(short, long, default_value = "clipboard")]
    pub output_dest: OutputDestination,

    /// Type of input
    #[arg(short, long, default_value = "markdown")]
    pub kind: InputKind,

    /// Max line width
    #[arg(short, long, default_value = "80")]
    pub width: usize,

    /// Keep bold and italic markdown formatting (by default, these are stripped)
    #[arg(long, default_value = "false")]
    pub keep_emphasis: bool,
}
