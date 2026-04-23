use clap::{CommandFactory, Parser, Subcommand};
use std::{fs, process::exit};
use venv::VenvManager;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Manage Python virtual environments from a small shell-friendly CLI.",
    long_about = None
)]
pub struct CLI {
    #[arg(value_name = "NAME", help = "Activate a venv by name")]
    activate: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
enum Commands {
    /// List available virtual environments
    #[command(short_flag('l'))]
    List,

    /// Print only venv names, one per line
    #[command(hide = true)]
    ListNames,

    /// Activate a virtual environment
    #[command(short_flag('a'), arg_required_else_help(true))]
    Activate {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Create a new virtual environment
    #[command(short_flag('c'), arg_required_else_help(true))]
    Create {
        #[arg(value_name = "NAME")]
        name: String,
        #[arg(value_name = "PYTHON_VERSION", help = "Examples: 3.12 or python3.12")]
        version: Option<String>,
    },

    /// Delete a virtual environment
    #[command(short_flag('d'), arg_required_else_help(true))]
    Delete {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Generate shell completions
    #[clap(alias = "--generate-shell-completion", hide = true)]
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    // Get the program options
    let cli = CLI::parse();

    let venv_manager: venv::VenvManager;
    if let Ok(x) = VenvManager::new() {
        venv_manager = x;
    } else {
        exit(1);
    }

    // if there's no command, but an arg, try activating the env
    if let Some(name) = cli.activate {
        let path = venv_manager.venv_store.to_string_lossy();
        if let Some(cmd) = venv_manager.activate(name) {
            write_cmd(path.as_ref(), cmd);
        } else {
            exit(3);
        }
    }
    // if there's a command run that
    else if let Some(cmd) = cli.command {
        let generated_cmd = match cmd {
            Commands::List => {
                venv_manager.list();
                None
            }
            Commands::ListNames => {
                for name in venv_manager.list_names() {
                    println!("{name}");
                }
                None
            }
            Commands::Activate { name } => venv_manager.activate(name),
            Commands::Create { name, version } => venv_manager.create(name, version),
            Commands::Delete { name } => venv_manager.delete(name),

            // e.g. `$ cli completions bash`
            Commands::Completions { shell } => {
                shell.generate(&mut CLI::command(), &mut std::io::stdout());
                None
            }
        };
        if let Some(cmd) = generated_cmd {
            let pth = venv_manager.venv_store.to_str().unwrap();
            write_cmd(pth, cmd);
            exit(0);
        }
        exit(0);
    }
    // default to interactive mode
    else {
        // interactive mode
        if let Some(cmd) = venv_manager.interactive() {
            let pth = venv_manager.venv_store.to_str().unwrap();
            write_cmd(pth, cmd);
        } else {
            // exit and don't run the command
            exit(3);
        }
    }
}

fn write_cmd(path: &str, cmd: String) {
    let hist_path = format!("{path}/.history");
    fs::write(hist_path, cmd).expect(format!("Error writing to file {}", path).as_str());
}
