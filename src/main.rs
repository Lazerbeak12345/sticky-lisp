use clap::Parser;
use std::{path::PathBuf,fs::read_to_string};
use inkwell::{context::Context,passes::PassManager};

/** Compile a sticky lisp program.
 *
 * The CLI api will follow semver. Breaking changes may happen, but the version
 * arg will remain.
 */
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the input file.
    #[clap(parse(from_os_str))]
    path: PathBuf
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    println!("Input file: {}", path.display());
    let file = match read_to_string(&path) {
        Err(reason) => {
            println!("Could not open input file `{}`. {}", &path.display(), reason);
            return ()
        },
        Ok(file) => file
    };
    //println!("Content '{}'", file);
    let _sexp = match lexpr::from_str(file.as_str()) {
        Err(reason) => {
            println!("Failed to parse sexp from file `{}`. {}", &path.display(), reason);
            return ()
        },
        Ok(sexp) => sexp,
    };
    todo!("sexp to scheme AST?");
    todo!("ast to LLVM calls");
    //print!("Do something with {}", sexp)
    /*let context = Context::create();
    let module = context.create_module("sticky lisp");
    let builder = context.create_builder();*/
}
