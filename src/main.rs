mod filters;
mod args;

use args::TruthArgs;
use clap::Parser;


fn main() {
    let args = TruthArgs::parse();
    // filters::manager_sal();
    println!("args: {:?}", args);
}


