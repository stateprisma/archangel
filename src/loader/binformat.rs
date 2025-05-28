use goblin;

use super::bininfo::{Arch, BinInfo, Format};

pub enum BinFormat<'a> {
    Elf(goblin::elf::Elf<'a>),
    PE(goblin::pe::PE<'a>),
    Mach(goblin::mach::Mach<'a>),
}

impl<'a> BinInfo for BinFormat<'a> {
    fn entry_point(&self) -> u64 {
        match self {
            BinFormat::Elf(elf) => elf.entry_point(),
            BinFormat::PE(pe) => pe.entry_point(),
            BinFormat::Mach(mach) => mach.entry_point(),
        }
    }

    fn architecture(&self) -> Arch {
        match self {
            BinFormat::Elf(elf) => elf.architecture(),
            BinFormat::PE(pe) => pe.architecture(),
            BinFormat::Mach(mach) => mach.architecture(),
        }
    }

    fn format(&self) -> Format {
        match self {
            BinFormat::Elf(elf) => elf.format(),
            BinFormat::PE(pe) => pe.format(),
            BinFormat::Mach(mach) => mach.format(),
        }
    }

    fn is_little_endian(&self) -> bool {
        match self {
            BinFormat::Elf(elf) => elf.is_little_endian(),
            BinFormat::PE(pe) => pe.is_little_endian(),
            BinFormat::Mach(mach) => mach.is_little_endian(),
        }
    }

    fn text_section(&self) -> (u64, usize) {
        match self {
            BinFormat::Elf(elf) => elf.text_section(),
            BinFormat::PE(pe) => pe.text_section(),
            BinFormat::Mach(mach) => mach.text_section(),
        }
    }
    fn va_to_offset(&self, va: u64) -> usize {
        match self {
            BinFormat::Elf(elf) => elf.va_to_offset(va),
            BinFormat::PE(pe) => pe.va_to_offset(va),
            BinFormat::Mach(mach) => mach.va_to_offset(va),
        }
    }
}
