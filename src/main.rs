use clap::{Parser, Subcommand, ValueEnum};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "cli-project")]
#[command(version)]
#[command(about = "A simple CLI with foo, bar, baz, and animal commands")]
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
    /// Print a cute animal
    Animal {
        /// The type of animal to print
        #[arg(value_enum)]
        animal: AnimalType,
    },
    /// Interactive tutorial for learning about all commands
    Tutorial,
}

#[derive(ValueEnum, Clone, Debug)]
enum AnimalType {
    /// A cute cat
    Cat,
    /// A cute bird
    Bird,
    /// A cute snake
    #[clap(alias = "snek")]
    Snake,
}

fn wait_for_enter() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn run_tutorial() {
    println!("Welcome to the CLI Project Tutorial!");
    println!("=====================================\n");

    println!("This tutorial will teach you about all the available commands.");
    println!("Let's get started!\n");
    wait_for_enter();

    println!("\n--- Command 1: foo ---");
    println!("The 'foo' command is the simplest command.");
    println!("It just prints 'foo command was run' to the console.\n");
    println!("Example:");
    println!("  $ cli-project foo");
    println!("  foo command was run\n");
    wait_for_enter();

    println!("\n--- Command 2: bar ---");
    println!("The 'bar' command is similar to foo.");
    println!("It prints 'bar command was run' to the console.\n");
    println!("Example:");
    println!("  $ cli-project bar");
    println!("  bar command was run\n");
    wait_for_enter();

    println!("\n--- Command 3: baz ---");
    println!("The 'baz' command is the third simple command.");
    println!("It prints 'baz command was run' to the console.\n");
    println!("Example:");
    println!("  $ cli-project baz");
    println!("  baz command was run\n");
    wait_for_enter();

    println!("\n--- Command 4: animal ---");
    println!("The 'animal' command is more interesting!");
    println!("It prints ASCII art of a cute animal.");
    println!("You can choose from: cat, bird, or snake.\n");
    println!("Examples:");
    println!("  $ cli-project animal cat");
    println!("   /\\_/\\");
    println!("  ( o.o )");
    println!("   > ^ <\n");
    println!("  $ cli-project animal snake");
    println!("       ____");
    println!("      / o o\\");
    println!("     |  >   |");
    println!("      \\____/");
    println!("         |");
    println!("       __|");
    println!("      | ...\n");
    wait_for_enter();

    println!("\n--- Command 5: tutorial ---");
    println!("The 'tutorial' command (which you're using right now!)");
    println!("is an interactive guide that teaches you about all the other commands.\n");
    println!("Example:");
    println!("  $ cli-project tutorial\n");
    wait_for_enter();

    println!("\n=====================================");
    println!("Tutorial complete! You now know about all the commands:");
    println!("  - foo, bar, baz: Simple commands that print messages");
    println!("  - animal: Prints cute ASCII art animals");
    println!("  - tutorial: This interactive guide");
    println!("\nHappy coding!");
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
        Commands::Animal { animal } => match animal {
            AnimalType::Cat => {
                println!(" /\\_/\\");
                println!("( o.o )");
                println!(" > ^ <");
            }
            AnimalType::Bird => {
                println!("   \\");
                println!("  (\\");
                println!("_o/\\_");
            }
            AnimalType::Snake => {
                println!("       ____");
                println!("      / o o\\");
                println!("     |  >   |");
                println!("      \\____/");
                println!("         |");
                println!("       __|");
                println!("      |");
                println!("      |__");
                println!("         |");
                println!("       __|");
                println!("      |");
                println!("      |__");
                println!("         |");
                println!("        /");
                println!("       /");
                println!("      /");
                println!("     /");
                println!("    /");
                println!("   |");
                println!("   |");
                println!("  /");
                println!(" /");
            }
        },
        Commands::Tutorial => {
            run_tutorial();
        }
    }
}
