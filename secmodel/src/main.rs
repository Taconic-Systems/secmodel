use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        long,
        short,
        env = "SECMODEL",
        default_value = "secmodel.toml",
        help = "The security model to use"
    )]
    model: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check {},
    Mermaid {},
    Report {},
}

fn main() {
    let cli = Cli::parse();
    let model = match secmodel_core::load(&cli.model) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Unable to load model {}: {e}", cli.model);
            std::process::exit(1);
        }
    };

    match &cli.command {
        Commands::Check {} => {
            println!("{}", serde_json::to_string_pretty(&model).unwrap());

            std::process::exit(0);
        }
        Commands::Mermaid {} => {
            println!(
                "{}",
                secmodel_mermaid::Render::render(&model, "", &model).unwrap()
            );
            std::process::exit(0);
        }
        Commands::Report {} => {
            println!("{}", secmodel_md::Report::report(&model, &model));
            std::process::exit(0);
        }
    }
}
