mod cli;

use std::{fs, io, process};

use cli::Args;
use pulldown_cmark::{html, Parser};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let text = fs::read_to_string(&args.path)?;
    let parser = Parser::new(&text);

    let mut content = String::new();
    html::push_html(&mut content, parser);

    // I'm too lazy to do this myself, and apparently there's not an option for it via pulldown...
    let content = content.replace("--", "&mdash;");

    if let Some(output) = &args.output {
        fs::write(output, content)?;
    } else {
        println!("{content}");
    }

    Ok(())
}
