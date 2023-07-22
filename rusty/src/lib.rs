use home;
use interactive::MenuItem;
use std::{
    fs,
    io::{self, Write},
    path,
};

use crossterm::{cursor, execute, style::Stylize, terminal};
mod interactive;

const VENV_STORE: &str = ".venv";
// const VENV_STORE: &str = "VENV_TEMP";
pub struct VenvManager {
    pub venv_store: path::PathBuf,
}

impl VenvManager {
    pub fn new() -> Result<VenvManager, ()> {
        let mut home_dir: path::PathBuf;
        if let Some(pth) = home::home_dir() {
            home_dir = pth;
        } else {
            eprintln!("Unable to get your home dir");
            return Err(());
        }

        home_dir.push(VENV_STORE);
        if !home_dir.exists() {
            eprintln!("creating {}", home_dir.to_str().unwrap());
            fs::create_dir(home_dir.as_path()).expect("Error creating new .venv dir");
        }
        return Ok(VenvManager {
            venv_store: home_dir,
        });
    }

    /// run in interactive mode
    pub fn interactive(&self) -> Option<String> {
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
        if choice as usize > menu.menu_items.len() {
            return None;
        }

        let cmd = match choice {
            0 => self.activate(),
            1 => self.create(),
            2 => self.delete(),
            _ => None,
        };

        return cmd;
    }

    /// display another menu allowing the user to choose from availabel venv's
    /// in the venv dir
    pub fn activate(&self) -> Option<String> {
        let menu = self.get_venv_vec();
        if menu.len() == 0 {
            eprintln!("No venvs found");
            return None;
        }

        // make a new menu
        let mut menu = interactive::Menu {
            prompt: "Choose and option".to_string(),
            cursor_pos: 0,
            menu_items: menu,
        };

        // ask the user to select the venv from the menu
        let choice = menu.display();
        if choice as usize > menu.menu_items.len() {
            return None;
        }

        // return the env to activate
        let cmd = format!(
            "source {}/{}/bin/activate",
            self.venv_store.to_str().unwrap(),
            menu.menu_items[choice as usize].text.clone()
        );
        return Some(cmd);
    }

    /// Create a new venv from the user's input name
    pub fn create(&self) -> Option<String> {
        // enter alt screen
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
        )
        .unwrap();

        // get name from user
        eprint!("Name of venv: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        name = name.replace("\n", "");
        if name.is_empty() {
            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            eprintln!("venv name can't be blank");
            return None;
        }

        // ask if the user wants to activate it now
        let menu = vec![
            interactive::MenuItem {
                text: "Yes, activate".to_string(),
            },
            interactive::MenuItem {
                text: "No".to_string(),
            },
        ];

        let mut menu = interactive::Menu {
            prompt: "Activate the new venv?".to_string(),
            cursor_pos: 0,
            menu_items: menu,
        };

        let choice = menu.display();

        let rtn = match choice {
            // yes, activate
            0 => Some(format!(
                " python3 -m venv {}/{} && source {}/{}/bin/activate",
                self.venv_store.to_str().unwrap(),
                name,
                self.venv_store.to_str().unwrap(),
                name
            )),
            // just create
            1 => Some(format!(
                "python3 -m venv {}/{}",
                self.venv_store.to_str().unwrap(),
                name
            )),
            _ => None,
        };

        execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        rtn
    }

    pub fn delete(&self) -> Option<String> {
        // enter alt screen
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
        )
        .unwrap();

        let menu = self.get_venv_vec();
        if menu.len() == 0 {
            eprintln!("No venvs found");
            return None;
        }
        // make a new menu
        let mut menu = interactive::Menu {
            prompt: "Choose a venv".to_string(),
            cursor_pos: 0,
            menu_items: menu,
        };

        // ask the user to select the venv from the menu
        let choice = menu.display();
        if choice as usize > menu.menu_items.len() {
            return None;
        }

        // return the env to activate
        let cmd = format!(
            "rm -rf {}/{}",
            self.venv_store.to_str().unwrap(),
            menu.menu_items[choice as usize].text.clone()
        );

        execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        // create the command to delete the folder holding the venv
        Some(cmd)
    }

    fn get_venv_vec(&self) -> Vec<MenuItem> {
        // put the available venv's in a menu
        let mut menu = Vec::new();
        let envs = fs::read_dir(&self.venv_store).unwrap();
        for f in envs {
            let name = f.unwrap().file_name();
            let name = name.to_str().unwrap().to_string();
            if name == ".history" {
                continue;
            }
            menu.push(interactive::MenuItem { text: name })
        }
        menu
    }

    pub fn list(&self) {
        let a = "Available venvs:";
        eprintln!("{}", a.blue());
        let d = fs::read_dir(&self.venv_store).unwrap();
        for f in d {
            let f_name = f.unwrap().file_name();
            let f_name = f_name.to_str().unwrap();
            if f_name == ".history" {
                continue;
            }
            let fmt = format!("{}", f_name);
            eprintln!("  {}", fmt.yellow());
        }
    }
}
