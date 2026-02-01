use clap::{Parser, Subcommand, ValueEnum};

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
    }
}
