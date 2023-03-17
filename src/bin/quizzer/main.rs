use clap::{Parser, Subcommand};

mod input;
mod play;
mod validators;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Input,
    Play
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Input => input::input_mode(),
        Command::Play => play::play_mode()
    }
}
