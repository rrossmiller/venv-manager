use clap::{CommandFactory, Parser, Subcommand};
use std::process::exit;
use venv::VenvManager;
pub mod interactive;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command()]
    activate: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
enum Commands {
    /// List your projects
    #[command(short_flag('l'))]
    List,

    /// Add an alias to a project
    #[command(short_flag('a'), arg_required_else_help(true))]
    Add {
        #[arg(value_name = "NAME{:?}")]
        name: String,
        #[arg(value_name = "PATH{:?}")]
        path: String,
    },

    /// Disactivates an alias
    #[command(short_flag('t'), arg_required_else_help(true))]
    Toggle { name: String },

    /// Deletes an alias
    #[command(short_flag('d'), arg_required_else_help(true))]
    Delete { name: String },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    // Get the program options
    let cli = CLI::parse();

    let venv: venv::VenvManager;
    if let Ok(x) = VenvManager::new() {
        venv = x;
    } else {
        exit(1);
    }

    // if there's a command run that
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => venv.list(),
            Commands::Add { name, path } => {
                eprintln!("add{:?}", venv.venv_store);
            }

            Commands::Toggle { name } => {
                eprintln!("toggle{:?}", venv.venv_store);
            }

            Commands::Delete { name } => {
                eprintln!("del{:?}", venv.venv_store);
            }
            // e.g. `$ cli completions bash`
            Commands::Completions { shell } => {
                shell.generate(&mut CLI::command(), &mut std::io::stdout());
            }
        }
    }
    // if there's no command, but an arg, try activating the env
    else if let Some(env) = cli.activate {
        let pth = venv.venv_store.to_str().unwrap();
        println!(".{pth}/{env}/bin/activate");
    } else {
        //interactive mode
        println!("interactive");
        interactive::start();
    }
}
