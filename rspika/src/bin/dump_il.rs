use anyhow::Context;
use chumsky::Parser;
use rspika::ast::module;
use rspika::token::tokenize;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os();
    let infile = args.nth(1).context("missing argument: INFILE")?;
    let raw_module = std::fs::read_to_string(infile).context("cannot read input file")?;

    let tokens = match tokenize().parse(raw_module) {
        Ok(x) => x,
        Err(errs) => {
            for err in errs {
                eprintln!("{:?}", err);
            }
            return Ok(());
        }
    };
    let ast = match module().parse(tokens) {
        Ok(x) => x,
        Err(errs) => {
            for err in errs {
                eprintln!("{:?}", err);
            }
            return Ok(());
        }
    };

    let il = ast.visit_il();

    for (name, func) in &il.functions {
        println!();
        println!("{}()", name);
        for (i, instr) in func.assembly.instructions().iter().enumerate() {
            for label in func.assembly.labels_at(i) {
                println!("  {:?}:", label);
            }
            println!("    {:?}", instr);
        }
    }

    Ok(())
}
