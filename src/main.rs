use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli-project")]
#[command(about = "A simple CLI with foo, bar, and baz commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the foo command
    Foo,
    /// Run the bar command
    Bar,
    /// Run the baz command
    Baz,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Foo => {
            println!("foo command was run");
        }
        Commands::Bar => {
            println!("bar command was run");
        }
        Commands::Baz => {
            println!("baz command was run");
        }
    }
}
