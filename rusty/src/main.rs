use clap::{CommandFactory, Parser, Subcommand};
use std::{fs, process::exit};
use venv::VenvManager;
mod interactive;

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
    println!("JUST COPY WHAT THE GO VERSION DOES WITH FILE, EXCEPT DON'T RELY ON SOME SCRIPT IN .ZSHRC -- THE SCRIPT CAN BE IN /BIN AND call a the bin installed in some install dir (.venv probs)");
    // Get the program options
    let cli = CLI::parse();

    let venv_manager: venv::VenvManager;
    if let Ok(x) = VenvManager::new() {
        venv_manager = x;
    } else {
        exit(1);
    }

    // if there's no command, but an arg, try activating the env
    if let Some(env) = cli.activate {
        let path = venv_manager.venv_store.to_str().unwrap();
        write_cmd(path, env);
    }
    // if there's a command run that
    else if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => venv_manager.list(),
            Commands::Add { name, path } => {
                eprintln!("add{:?}", venv_manager.venv_store);
            }

            Commands::Delete { name } => {
                eprintln!("del{:?}", venv_manager.venv_store);
            }
            // e.g. `$ cli completions bash`
            Commands::Completions { shell } => {
                shell.generate(&mut CLI::command(), &mut std::io::stdout());
            }
        }
    }
    // default to interactive mode
    else {
        //interactive mode
        // interactive(&venv_manager);
        let opt = venv_manager.interactive();
        if opt.is_none() {
            exit(0);
        }
        if let Some(env) = opt {
            let pth = venv_manager.venv_store.to_str().unwrap();
            write_cmd(pth, env);
        }
    }

    println!("\n***RUNNING LAST LINE IN HIST FILE***");
}

fn write_cmd(path: &str, env: String) {
    let hist_path = format!("{path}/history");
    let cmd = format!("source {path}/{env}/bin/activate");
    fs::write(hist_path, cmd).expect(format!("Error writing to file {}", path).as_str());
}
