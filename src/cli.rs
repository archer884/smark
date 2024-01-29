use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// markdown file
    pub path: String,

    /// output file
    #[arg(short, long)]
    pub output: Option<String>,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
