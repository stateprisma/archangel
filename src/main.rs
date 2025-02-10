use clap::Parser;

use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum Arch {
    #[default]
    X86_64,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum OS {
    #[default]
    Linux,
    Windows,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum FileFormat {
    #[default]
    Object,
    PE,
    Elf,
}

impl core::fmt::Display for Arch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An x64 to ARM64 binary recompiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to binary file to recompile
    #[arg(short, long)]
    bin: String,

    /// Binary format
    #[arg(short, long)]
    format: FileFormat,

    /// Architecture of binary
    #[arg(short, long)]
    arch: Arch,

    /// Operating system
    #[arg(short, long)]
    os: OS,
}

fn main() {
    let _args = Args::parse();
}
