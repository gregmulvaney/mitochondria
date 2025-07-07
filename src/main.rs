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
    // TODO: Convert to vector for multipackage installs
    Install { package: String },
    List {},
    Bobr {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package } => {
            println!("Installing package {package:?}");
        }
        Commands::List {} => {
            println!("Heres your packages");
        }
        Commands::Bobr {  } => {
            print!("{}", crate::bobr::BOBR);
        }
    }
}
