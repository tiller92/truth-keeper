use clap:: {
  Args,
  Parser,
  Subcommand,
};

#[derive(Debug,Parser)]
#[clap(author,version,about)]

pub struct TruthArgs {
  /// first argument!
  pub first_arg: String,
  /// second argument!
  pub second_arg: String,
  /// third argument!
  pub third_arg: String,
}