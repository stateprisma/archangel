use goblin;

use goblin::elf::Elf;
use goblin::mach::{Mach, constants::cputype::*, load_command};
use goblin::pe::PE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Arch {
    X86_32,
    X86_64,
    ARM,
    ARM64,
    MIPS,
    PPC,
    PPC64,
    RISCV,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    ELF,
    PE,
    MachO,
}

pub trait BinInfo {
    fn entry_point(&self) -> u64;
    fn architecture(&self) -> Arch;
    fn format(&self) -> Format;
    fn is_little_endian(&self) -> bool;
    fn text_section(&self) -> (u64, usize);
    fn va_to_offset(&self, va: u64) -> usize;
}

impl<'a> BinInfo for Elf<'a> {
    fn entry_point(&self) -> u64 {
        self.entry
    }
    fn architecture(&self) -> Arch {
        match self.header.e_machine {
            goblin::elf::header::EM_X86_64 => Arch::X86_64,
            goblin::elf::header::EM_386 => Arch::X86_32,
            _ => Arch::Unknown,
        }
    }
    fn format(&self) -> Format {
        Format::ELF
    }
    fn is_little_endian(&self) -> bool {
        self.little_endian
    }
    fn text_section(&self) -> (u64, usize) {
        let shdr = self
            .section_headers
            .iter()
            .find(|s| self.shdr_strtab.get_at(s.sh_name) == Some(".text"))
            .expect("No .text section found in ELF");
        (shdr.sh_addr, shdr.sh_size as usize)
    }
    fn va_to_offset(&self, va: u64) -> usize {
        let shdr = self
            .section_headers
            .iter()
            .find(|s| s.sh_addr <= va && va < s.sh_addr + s.sh_size)
            .expect("VA not in any ELF section");
        (va - shdr.sh_addr + shdr.sh_offset) as usize
    }
}

impl<'a> BinInfo for PE<'a> {
    fn entry_point(&self) -> u64 {
        self.entry as u64 + self.image_base as u64
    }
    fn architecture(&self) -> Arch {
        match self.header.coff_header.machine {
            goblin::pe::header::COFF_MACHINE_X86 => Arch::X86_32,
            goblin::pe::header::COFF_MACHINE_X86_64 => Arch::X86_64,
            _ => Arch::Unknown,
        }
    }
    fn format(&self) -> Format {
        Format::PE
    }
    fn is_little_endian(&self) -> bool {
        true
    }
    fn text_section(&self) -> (u64, usize) {
        let sec = self
            .sections
            .iter()
            .find(|s| s.name().unwrap_or_default() == ".text")
            .expect("No .text section found in PE");
        (
            sec.virtual_address as u64 + self.image_base as u64,
            sec.size_of_raw_data as usize,
        )
    }
    fn va_to_offset(&self, va: u64) -> usize {
        let sec = self
            .sections
            .iter()
            .find(|s| {
                let sec_va = s.virtual_address as u64 + self.image_base as u64;
                sec_va <= va && va < sec_va + s.virtual_size as u64
            })
            .expect("VA not in any PE section");
        (va - (sec.virtual_address as u64 + self.image_base as u64)
            + sec.pointer_to_raw_data as u64) as usize
    }
}

impl<'a> BinInfo for Mach<'a> {
    fn entry_point(&self) -> u64 {
        match self {
            Mach::Binary(bin) => {
                for cmd in &bin.load_commands {
                    if let load_command::CommandVariant::Main(main) = &cmd.command {
                        let vmaddr = bin.segments.get(0).expect("No segments").vmaddr;
                        return main.entryoff + vmaddr;
                    }
                }
                todo!("No LC_MAIN found in Mach-O");
            }
            Mach::Fat(_) => todo!("Fat Mach-O binaries are not supported yet"),
        }
    }

    fn architecture(&self) -> Arch {
        match self {
            Mach::Binary(bin) => match bin.header.cputype() {
                CPU_TYPE_X86 => Arch::X86_32,
                CPU_TYPE_X86_64 => Arch::X86_64,
                CPU_TYPE_ARM => Arch::ARM,
                CPU_TYPE_ARM64 => Arch::ARM64,
                CPU_TYPE_POWERPC => Arch::PPC,
                CPU_TYPE_POWERPC64 => Arch::PPC64,
                _ => Arch::Unknown,
            },
            Mach::Fat(_) => todo!("Fat Mach-O binaries are not supported yet"),
        }
    }

    fn format(&self) -> Format {
        Format::MachO
    }

    fn is_little_endian(&self) -> bool {
        match self {
            Mach::Binary(bin) => bin.little_endian,
            Mach::Fat(_) => todo!("Fat Mach-O binaries are not supported yet"),
        }
    }
    fn text_section(&self) -> (u64, usize) {
        todo!("MachO-s are not supported yet");
    }
    fn va_to_offset(&self, va: u64) -> usize {
        let _ = va;
        todo!("MachO-s are not supported yet");
    }
}
