use clap::Parser;
use loader::bininfo::BinInfo;

mod decoder;
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
    println!(
        " - Endianness:   {}",
        match binfile.is_little_endian() {
            true => "Little Endian",
            _ => "Big Endian",
        }
    );
    println!(" - Bin format:   {:?}", binfile.format());
    println!(" - Entry point:  0x{:x}", binfile.entry_point());

    // Disassemble code
    let root = decoder::codeblock::CodeBlock::disassemble_from_entry(&binfile, mmap);
    decoder::codeblock::CodeBlock::pretty_print(&root);

    // Free mmap
    loader::binload::free_mmap(mmap);
}
