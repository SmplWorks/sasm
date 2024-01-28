use std::io::Write;

use clap::Parser;
use sasm_lib::{compile, utils::{Error, Result}};

/// Assembler for SimpleASM
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file to assemble
    in_path : String,

    /// Output file
    #[arg(short = 'o', default_value = "main.bin")]
    out_path : String,
}

fn compile_file(fpath : &str) -> Result<Vec<u8>> {
    compile(&std::fs::read_to_string(fpath).map_err(|err| Error::External(err.to_string()))?)
}

fn write_file(fpath : &str, bytes : &[u8]) -> Result<()> {
    let mut fout = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(fpath)
        .map_err(|err| Error::External(err.to_string()))?;
    fout.write_all(bytes)
        .map_err(|err| Error::External(err.to_string()))?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bytes = compile_file(&args.in_path)?;
    write_file(&args.out_path, &bytes)?;

    Ok(())
}
