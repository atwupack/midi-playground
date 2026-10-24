use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{info, LevelFilter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Listen { port: String }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter_level(LevelFilter::Debug).try_init()?;
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Listen { port } => {
            info!("'myapp listen' was used, port is: {port:?}");
            listen(port);
        }
    }
    Ok(())
}

fn listen(port: &String) {
    
}