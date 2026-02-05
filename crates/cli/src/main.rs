use clap::Parser;
#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Option<Command>,
}
#[derive(clap::Subcommand)]
enum Command {
    Export { widget: String },
    Validate { file: String },
}
fn main() {
    let args = Args::parse();
    match args.cmd {
        Some(Command::Export { widget }) => println!(\"Export widget: {}\", widget),
        Some(Command::Validate { file }) => println!(\"Validate config: {}\", file),
        None => println!(\"unkai CLI\"),
    }
}
