use clap::{CommandFactory, Parser, Subcommand};
use std::{fs, process::exit};
use venv::VenvManager;

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
    Activate {
        #[arg(value_name = "NAME{:?}")]
        name: String,
    },

    /// Creates a new venv
    #[command(short_flag('c'), arg_required_else_help(true))]
    Create {
        name: String,
        version: Option<String>,
    },

    /// Deletes a venv
    #[command(short_flag('d'), arg_required_else_help(true))]
    Delete { name: String },

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
        let path = venv_manager.venv_store.to_str().unwrap();
        let cmd = format!("source {path}/{name}/bin/activate");
        write_cmd(path, cmd);
    }
    // if there's a command run that
    else if let Some(cmd) = cli.command {
        let x = match cmd {
            Commands::List => {
                venv_manager.list();
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
        if let Some(cmd) = x {
            let pth = venv_manager.venv_store.to_str().unwrap();
            write_cmd(pth, cmd);
        } else {
            exit(3);
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
