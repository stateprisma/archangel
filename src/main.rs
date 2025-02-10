use clap::Parser;
use object::Object;

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
    let args = Args::parse();
    let (mmap, binfile) =
        loader::binload::load_file(&args.bin).expect("Could not load binary file");

    println!("File info:");
    println!(" - Architecture: {:?}", binfile.architecture());
    println!(" - Endianness:   {:?}", binfile.endianness());
    println!(" - Bin format:   {:?}", binfile.format());

    // Free mmap
    loader::binload::free_mmap(mmap);
}
