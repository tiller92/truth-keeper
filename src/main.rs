mod filters;
mod args;

use args::TruthArgs;
use clap::Parser;


fn main() {
    let args = TruthArgs::parse();

    filters::managers(&args.file_path);
}


