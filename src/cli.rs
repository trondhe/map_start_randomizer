use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub name: Option<String>,
}
