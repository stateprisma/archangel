use crate::loader::binformat::BinFormat;
use crate::loader::bininfo::{Arch, BinInfo};
use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

type CodeBlockRef = Arc<Mutex<CodeBlock>>;

pub static BLOCKS: Lazy<Mutex<HashMap<u64, CodeBlockRef>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct CodeBlock {
    pub start_ip: u64,
    pub instructions: Vec<Instruction>,
    pub successors: Vec<CodeBlockRef>,
}

impl CodeBlock {
    fn new(start_ip: u64) -> Self {
        Self {
            start_ip,
            instructions: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn disassemble_from_entry(bin: &BinFormat, bytes: &[u8]) -> CodeBlockRef {
        let bitness = match bin.architecture() {
            Arch::X86_64 => 64,
            Arch::X86_32 => 32,
            _ => panic!("Unsupported architecture"),
        };

        let entry_ip = bin.entry_point();
        let root = Self::get_or_create(entry_ip);
        Self::recursive_disasm(root.clone(), bin, bytes, bitness);
        root
    }

    fn recursive_disasm(block: CodeBlockRef, bin: &BinFormat, bytes: &[u8], bitness: u32) {
        let start_ip = block.lock().unwrap().start_ip;
        let mut decoder = Decoder::with_ip(
            bitness,
            &bytes[bin.va_to_offset(start_ip)..],
            start_ip,
            DecoderOptions::NONE,
        );

        let mut instr = Instruction::default();

        while decoder.can_decode() {
            decoder.decode_out(&mut instr);
            block.lock().unwrap().instructions.push(instr.clone());

            if instr.is_jmp_short_or_near() || instr.is_call_near() {
                let target = instr.near_branch_target();
                let child = Self::get_or_create(target);
                block.lock().unwrap().successors.push(child.clone());

                if child.lock().unwrap().instructions.is_empty() {
                    Self::recursive_disasm(child, bin, bytes, bitness);
                }

                /* Stop disassembling this block if it never returns */
                if instr.is_jmp_short_or_near() {
                    break;
                }
            }

            if matches!(instr.mnemonic(), Mnemonic::Ret | Mnemonic::Int3) || instr.is_invalid() {
                break;
            }
        }
    }

    fn get_or_create(ip: u64) -> CodeBlockRef {
        let mut map = BLOCKS.lock().unwrap();
        map.entry(ip)
            .or_insert_with(|| Arc::new(Mutex::new(CodeBlock::new(ip))))
            .clone()
    }

    pub fn pretty_print(block: &CodeBlockRef) {
        let mut visited = std::collections::HashSet::new();
        Self::pretty_print_internal(block, &mut visited);
    }

    fn pretty_print_internal(block: &CodeBlockRef, visited: &mut HashSet<u64>) {
        let block_ip = block.lock().unwrap().start_ip;

        if !visited.insert(block_ip) {
            return;
        }

        let block = block.lock().unwrap();
        println!(
            "Block @ 0x{:X} ({} instructions)",
            block.start_ip,
            block.instructions.len()
        );
        for instr in &block.instructions {
            println!("  0x{:016X}: {}", instr.ip(), instr);
        }

        let successors = block.successors.clone();
        drop(block);

        for succ in successors {
            Self::pretty_print_internal(&succ, visited);
        }
    }
}
