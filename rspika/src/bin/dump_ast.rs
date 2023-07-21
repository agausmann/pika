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
    let module = match module().parse(tokens) {
        Ok(x) => x,
        Err(errs) => {
            for err in errs {
                eprintln!("{:?}", err);
            }
            return Ok(());
        }
    };

    println!("{:#?}", module);

    Ok(())
}
