use goblin;

use goblin::mach::constants::cputype::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl<'a> BinInfo for goblin::elf::Elf<'a> {
    fn entry_point(&self) -> u64 {
        self.entry
    }

    fn architecture(&self) -> Arch {
        match self.header.e_machine {
            0x03 => Arch::X86_32,
            0x3E => Arch::X86_64,
            0x28 => Arch::ARM,
            0xB7 => Arch::ARM64,
            0x08 => Arch::MIPS,
            0x14 => Arch::PPC,
            0xF3 => Arch::RISCV,
            _ => Arch::Unknown,
        }
    }

    fn format(&self) -> Format {
        Format::ELF
    }

    fn is_little_endian(&self) -> bool {
        self.little_endian
    }
}

impl<'a> BinInfo for goblin::pe::PE<'a> {
    fn entry_point(&self) -> u64 {
        self.entry as u64 + self.image_base as u64
    }

    fn architecture(&self) -> Arch {
        match self.header.coff_header.machine {
            0x014C => Arch::X86_32,
            0x8664 => Arch::X86_64,
            0x01C0 => Arch::ARM,
            0xAA64 => Arch::ARM64,
            _ => Arch::Unknown,
        }
    }

    fn format(&self) -> Format {
        Format::PE
    }

    fn is_little_endian(&self) -> bool {
        true
    }
}

impl<'a> BinInfo for goblin::mach::Mach<'a> {
    fn entry_point(&self) -> u64 {
        match self {
            goblin::mach::Mach::Binary(bin) => {
                for cmd in &bin.load_commands {
                    if let goblin::mach::load_command::CommandVariant::Main(main) = &cmd.command {
                        let vmaddr = bin.segments.get(0).expect("No segments").vmaddr;
                        return main.entryoff + vmaddr;
                    }
                }
                panic!("No LC_MAIN found in Mach-O");
            }
            goblin::mach::Mach::Fat(_) => panic!("Fat Mach-O binaries are not supported yet"),
        }
    }

    fn architecture(&self) -> Arch {
        match self {
            goblin::mach::Mach::Binary(bin) => match bin.header.cputype() {
                CPU_TYPE_X86 => Arch::X86_32,
                CPU_TYPE_X86_64 => Arch::X86_64,
                CPU_TYPE_ARM => Arch::ARM,
                CPU_TYPE_ARM64 => Arch::ARM64,
                CPU_TYPE_POWERPC => Arch::PPC,
                CPU_TYPE_POWERPC64 => Arch::PPC64,
                _ => Arch::Unknown,
            },
            goblin::mach::Mach::Fat(_) => panic!("Fat Mach-O binaries are not supported yet"),
        }
    }

    fn format(&self) -> Format {
        Format::MachO
    }

    fn is_little_endian(&self) -> bool {
        match self {
            goblin::mach::Mach::Binary(bin) => bin.little_endian,
            goblin::mach::Mach::Fat(_) => panic!("Fat Mach-O binaries are not supported yet"),
        }
    }
}
