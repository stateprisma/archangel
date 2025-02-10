use clap::Parser;

mod loader;

/// An x64 to ARM64 binary recompiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to binary file to recompile
    #[arg(short, long)]
    bin: String,
}

fn main() {
    let _args = Args::parse();
}
