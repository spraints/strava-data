mod args;
mod client;
mod login;
mod token;

use args::{Cli, Command};
use clap::Parser;

fn main() {
    match Cli::parse().command {
        Command::Login => login::main(),
        Command::Download(_) => println!("todo: implement login"),
    };
}
