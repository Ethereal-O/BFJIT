use clap::Parser;
use std::{fs::File, io::Read, io::Write, path::PathBuf};

use crate::bftype::bfcate::bfcate::VMArchType;
use crate::bftype::bferror;
use crate::bftype::bfwarn;

const STDIN: &str = "STDIN";
const STDOUT: &str = "STDOUT";

#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    #[clap(name = "FILE")]
    file_path: PathBuf,
    #[clap(short='i', long="input", help="input file or STDIN", default_value_t = String::from(STDIN))]
    input: String,
    #[clap(short='o', long="output", help="output file or STDOUT", default_value_t = String::from(STDOUT))]
    output: String,
}

pub struct StartArgs {
    vm_arch_type: VMArchType,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    str: String,
}

pub fn start_all(args: StartArgs) {
    let tokens_res = crate::bfparser::frontend::parser::parse(args.str.as_str());
    if tokens_res.is_err() {
        println!("{:?}", tokens_res.as_ref().unwrap_err());
        return;
    }
    let irs_res = crate::bfparser::frontend::ir::transfer_to_ir(&tokens_res.unwrap());
    if irs_res.is_err() {
        println!("{:?}", irs_res.as_ref().unwrap_err());
        return;
    }
    let asm_res =
        crate::bfparser::backend::codegen::gen_code(&irs_res.unwrap(), args.vm_arch_type.clone());
    if asm_res.is_err() {
        println!("{:?}", asm_res.as_ref().unwrap_err());
        return;
    }
    let vm_res = crate::bfvm::bfjit::vm::VMStruct::new(
        asm_res.unwrap(),
        args.input,
        args.output,
        args.vm_arch_type.clone(),
        false,
    );
    if vm_res.is_err() {
        println!("{:?}", "VMStruct::new error");
        return;
    }
    let tot_res = vm_res.unwrap().run();
    if tot_res.is_err() {
        println!("{:?}", tot_res.as_ref().unwrap_err());
        return;
    }
}

pub fn parse() -> Result<StartArgs, bferror::error::RuntimeError> {
    let opt = Opt::parse();
    let src = std::fs::read_to_string(opt.file_path);
    if src.is_err() {
        return Err(bferror::error::RuntimeError {
            index: 1,
            kind: bferror::error::RuntimeErrorKind::IO,
        });
    } else {
        let mut input: Box<dyn Read> = Box::new(std::io::stdin());
        let mut output: Box<dyn Write> = Box::new(std::io::stdout());
        if opt.input != STDIN {
            let input_res = File::open(opt.input);
            if input_res.is_err() {
                println!(
                    "{}",
                    bfwarn::warn::RuntimeWarn {
                        kind: bfwarn::warn::RuntimeWarnKind::ParseInputWarn,
                    }
                )
            } else {
                input = Box::new(input_res.unwrap());
            }
        }
        if opt.output != STDOUT {
            let output_res = File::create(opt.output);
            if output_res.is_err() {
                println!(
                    "{}",
                    bfwarn::warn::RuntimeWarn {
                        kind: bfwarn::warn::RuntimeWarnKind::ParseOutputWarn,
                    }
                )
            } else {
                output = Box::new(output_res.unwrap());
            }
        }
        return Ok(StartArgs {
            vm_arch_type: VMArchType::X64,
            input,
            output,
            str: src.unwrap(),
        });
    }
}
