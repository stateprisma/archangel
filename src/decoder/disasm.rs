use crate::loader::{binformat::BinFormat, bininfo::BinInfo};
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, Mnemonic, NasmFormatter};
use std::collections::HashSet;

const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;

pub fn disassemble_from_entry(
    bin: &BinFormat,
    bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let entry_addr = bin.entry_point();
    let bitness = match bin.architecture() {
        crate::loader::bininfo::Arch::X86_64 => 64,
        crate::loader::bininfo::Arch::X86_32 => 32,
        _ => panic!("Unsupported architecture for disassembly"),
    };

    let mut visited = HashSet::new();
    println!("\nDisassembly starting at entry point 0x{:X}:", entry_addr);
    recursive_disasm(entry_addr, bin, bytes, bitness, &mut visited)?;
    Ok(())
}

fn recursive_disasm(
    ip: u64,
    bin: &BinFormat,
    bytes: &[u8],
    bitness: u32,
    visited: &mut HashSet<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    if visited.contains(&ip) {
        return Ok(());
    }
    visited.insert(ip);
    println!("Visiting: 0x{:x}", ip);

    let offset = bin.va_to_offset(ip);
    let mut decoder = Decoder::with_ip(bitness, &bytes[offset..], ip, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::new();
    formatter.options_mut().set_digit_separator("");
    formatter.options_mut().set_first_operand_char_index(10);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        output.clear();
        formatter.format(&instruction, &mut output);

        print!("{:016X} ", instruction.ip());
        let instr_bytes = &bytes[bin.va_to_offset(instruction.ip())..][..instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);

        if instruction.is_jmp_short_or_near() || instruction.is_call_near() {
            let target = instruction.near_branch_target();
            recursive_disasm(target, bin, bytes, bitness, visited)?;
            if instruction.is_jmp_short_or_near() {
                break;
            }
        }

        if matches!(instruction.mnemonic(), Mnemonic::Ret | Mnemonic::Int3)
            || instruction.is_invalid()
        {
            break;
        }
    }

    Ok(())
}
