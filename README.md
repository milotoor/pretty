pretty
-------------

pretty is a basic Command Line Interface application for formatting raw text into pretty commit messages or SQL. In particular, the tool takes input from the user, parses it as [GitHub-Flavored Markdown](https://docs.github.com/en/github/writing-on-github/getting-started-with-writing-and-formatting-on-github/about-writing-and-formatting-on-github) and then reformats the message by wrapping lines at the parameterized maximum line length (default is 80 characters).

This project was primarily intended as a motivation for me to learn Rust. I commonly write thorough notes describing new features when I open a pull request on GitHub. When the PR is approved I will squash and merge the PR and provide an edited version of the PR description as my commit message. The only downside is that the PR description is written in Markdown and rendered on GitHub accordingly, while git works only with raw text. In the past I have made the commit messages more readable by manually truncating lines at 80 characters, but no more!

### Usage

Installation and usage are primitive for the time being:

```bash
$ git clone git@github.com:milotoor/pretty.git
$ cd pretty
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/pretty --help`
pretty 0.1.0

USAGE:
    pretty [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --width <width>    Max line width [default: 80]

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/pretty`
Enter a GitHub Flavored Markdown string to be reformatted.
- The string must not contain two (or more) consecutive newlines
- See https://github.github.com/gfm/ for the GFM spec 
- Press the Return key twice to indicate when the input has terminated.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin nec posuere enim. Fusce dignissim ultrices ultricies. Donec vel dui commodo, hendrerit sapien eu, volutpat libero. Quisque at dui metus.


Formatted output:
-----------------

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin nec posuere enim.
Fusce dignissim ultrices ultricies. Donec vel dui commodo, hendrerit sapien eu,
volutpat libero. Quisque at dui metus.
```

## Why Rust?

I wanted to learn Rust! There's no compelling technical reason, other than the basic fact that Rust is a pleasure to work with. Creating a CLI in Rust is straightforward, and [comrak](https://docs.rs/comrak/latest/comrak/) provides solid GFM parsing.
