use memmap2::{Mmap, MmapOptions};
use object::{self, File as ObjFile};
use std::{fs::File, io};

/// Loads a binary file and returns a leaked reference to `Mmap` and `object::File`.
pub fn load_file(path: &str) -> Result<(&'static Mmap, ObjFile<'static>), io::Error> {
    let file = File::open(path)?;

    // Make sure you free this memory chunk
    let leaked_mmap: &'static Mmap = Box::leak(Box::new(unsafe { MmapOptions::new().map(&file)? }));
    let obj_file = ObjFile::parse(&leaked_mmap[..])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse object file"))?;

    Ok((leaked_mmap, obj_file))
}

/// Frees the previously leaked `Mmap`.
pub fn free_mmap(leaked_mmap: &'static Mmap) {
    unsafe {
        drop(Box::from_raw(leaked_mmap as *const Mmap as *mut Mmap));
    }
}
