use clap:: {
  Parser,
};

#[derive(Debug,Parser)]
#[clap(author,version,about)]

pub struct TruthArgs {
  /// takes a file path or file name 
  pub file_path: String,
  
}