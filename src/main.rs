use clap::{Parser, Subcommand};

mod bobr;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(name = "Mitochondria")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install { package: Vec<String> },
    List {},
    Update {},
    Upgrade {},
    Remove { package: String },
    Bobr {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package } => {
            println!("Installing packages {:?}", package);
        }
        Commands::List {} => {
            println!("Heres your packages");
        }
        Commands::Update {} => {
            todo!()
        }
        Commands::Upgrade {} => {
            todo!()
        }
        Commands::Remove { package } => {
            println!("Removing package {package:?}")
        }
        Commands::Bobr {} => {
            print!("{}", crate::bobr::BOBR);
        }
    }
}
