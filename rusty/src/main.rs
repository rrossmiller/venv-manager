use clap::{CommandFactory, Parser, Subcommand};
use std::{fs, process::exit};
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
        let pth = venv_manager.venv_store.to_str().unwrap();
        write_cmd(
            format!("{pth}/history"),
            format!("source {pth}/{env}/bin/activate"),
        );
    }
    // if there's a command run that
    else if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => venv_manager.list(),
            Commands::Add { name, path } => {
                eprintln!("add{:?}", venv_manager.venv_store);
            }

            Commands::Toggle { name } => {
                eprintln!("toggle{:?}", venv_manager.venv_store);
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
        interactive();
    }
}

///////----
pub fn interactive() -> Option<String> {
    let menu = vec![
        interactive::MenuItem {
            text: "Activate".to_string(),
        },
        interactive::MenuItem {
            text: "Create".to_string(),
        },
        interactive::MenuItem {
            text: "Delete".to_string(),
        },
    ];

    let mut menu = interactive::Menu {
        prompt: "Choose and option".to_string(),
        cursor_pos: 0,
        menu_items: menu,
    };

    let choice = menu.display();

    println!("choice: {}", choice);
    // you're here
    let cmd = match choice {
        0 => activate(),
        1 => create(),
        2 => delete(),
        _ => "".to_string(),
    };

    return Some(String::from(cmd));
}

/// display another menu allowing the user to choose from availabel venv's
/// in the venv dir
fn activate() -> String {
    println!("activate");
    todo!();

    // get the available venv's

    // make a new menu

    // ask the user to select the venv from the menu

    // return the command to activate the venv

    "".to_string()
}

/// Create a new venv from the user's input name
fn create() -> String {
    println!("create");
    todo!();

    // get name from user

    // ask if the user wants to activate it now


    "".to_string()
}
fn delete() -> String {
    println!("delete");
    todo!();

    // get name from user

    // create the command to delete the folder holding the venv
    "".to_string()
}
fn write_cmd(path: String, cmd: String) {
    fs::write(path, cmd).unwrap();
}
