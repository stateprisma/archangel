use goblin::Object;
use memmap2::{Mmap, MmapOptions};
use std::{fs::File, io};

use super::binformat::BinFormat;

/// Loads a binary file and returns a leaked reference to `Mmap` and a parsed `BinFormat`.
pub fn load_file(path: &str) -> Result<(&'static Mmap, BinFormat<'static>), io::Error> {
    let file = File::open(path)?;
    let leaked_mmap: &'static Mmap = Box::leak(Box::new(unsafe { MmapOptions::new().map(&file)? }));

    let obj = match Object::parse(&leaked_mmap[..]) {
        Ok(obj) => obj,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported binary format",
            ));
        }
    };

    let bin = match obj {
        Object::Elf(elf) => BinFormat::Elf(elf),
        Object::PE(pe) => BinFormat::PE(pe),
        Object::Mach(mach) => BinFormat::Mach(mach),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported binary format",
            ));
        }
    };

    Ok((leaked_mmap, bin))
}

/// Frees the previously leaked `Mmap`.
pub fn free_mmap(leaked_mmap: &'static Mmap) {
    unsafe { drop(Box::from_raw(leaked_mmap as *const Mmap as *mut Mmap)) };
}
