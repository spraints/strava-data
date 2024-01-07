mod args;

use args::{Cli, Command};
use clap::Parser;

fn main() {
    match Cli::parse().command {
        Command::Login(_) => println!("todo: implement login"),
        Command::Download(_) => println!("todo: implement login"),
    };
}
