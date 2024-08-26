use home;
use interactive::MenuItem;
use std::{
    fs,
    io::{self, Write},
    path, process,
};

use crossterm::{cursor, execute, style::Stylize, terminal};
mod interactive;

const VENV_STORE: &str = ".venv";
// const VENV_STORE: &str = "VENV_TEMP";
/// Encapsulating struct for the venv manager
pub struct VenvManager {
    /// The path to the venv store. This is where the virtual environments and `VenvManager`
    /// management files are kept
    pub venv_store: path::PathBuf,
}

#[derive(Debug)]
struct PyVersion(String, String);

impl VenvManager {
    /// Create a new `VenvManager`
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

        Ok(VenvManager {
            venv_store: home_dir,
        })
    }

    /// Run in interactive mode
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

        let mut menu = interactive::Menu::new("Choose and option".to_string(), menu, false);
        let choice_opt = menu.display();
        let choice: usize;
        // testme
        if let Some(i) = choice_opt {
            choice = i;
        } else {
            return None;
        }

        let cmd = match choice {
            0 => self.activate_interactive(),
            1 => self.create_interactive(),
            2 => self.delete_interactive(),
            _ => None,
        };

        cmd
    }

    /// Send the command to activate the venv with the same name as `name`
    pub fn activate(&self, name: String) -> Option<String> {
        // return the env to activate
        let cmd = format!(
            "source {}/{}/bin/activate",
            self.venv_store.to_str().unwrap(),
            name
        );
        Some(cmd)
    }
    /// Display a menu allowing the user to choose from available venv's in the venv dir
    pub fn activate_interactive(&self) -> Option<String> {
        let menu = self.get_venv_vec();
        if menu.len() == 0 {
            eprintln!("No venvs found");
            return None;
        }

        // make a new menu
        let mut menu = interactive::Menu::new("Choose a venv".to_string(), menu, true);

        // ask the user to select the venv from the menu
        if let Some(choice) = menu.display() {
            if choice > menu.menu_items.len() {
                return None;
            }
            // return the env to activate
            return self.activate(menu.menu_items[choice as usize].text.clone());
        } else {
            return None;
        }
    }

    /// Create a new venv from the user's input name
    pub fn create(&self, name: String, version: Option<String>) -> Option<String> {
        let py_version = if let Some(v) = version {
            format!("python{}", v)
        } else {
            "python3".to_string()
        };
        let rtn = Some(format!(
            "{} -m venv {}/{}",
            py_version,
            self.venv_store.to_str().unwrap(),
            name
        ));

        eprintln!("Creating venv '{name}'");

        rtn
    }
    /// Create a new venv from the user's input name. The user will be asked for that new
    /// venv's name. The user can optionally activate the newly created venv.
    pub fn create_interactive(&self) -> Option<String> {
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
        // ask the user what version of python to use
        let versions = self.get_py_versions();
        let menu = versions
            .iter()
            .map(|v| interactive::MenuItem { text: v.0.clone() })
            .collect();
        let mut menu = interactive::Menu::new("Activate the new venv?".to_string(), menu, false);
        let idx = menu.display().unwrap();
        let py_ver = &versions[idx];

        // ask if the user wants to activate it now
        let menu = vec![
            interactive::MenuItem {
                // text: "Yes, activate".to_string(),
                text: format!("Yes, activate {} ({})", name, py_ver.0),
            },
            interactive::MenuItem {
                text: "No".to_string(),
            },
        ];

        let mut menu = interactive::Menu::new("Activate the new venv?".to_string(), menu, false);

        if let Some(choice) = menu.display() {
            let rtn = match choice {
                // yes, activate
                0 => Some(format!(
                    "{} -m venv {}/{} && source {}/{}/bin/activate",
                    py_ver.1,
                    self.venv_store.to_str().unwrap(),
                    name,
                    self.venv_store.to_str().unwrap(),
                    name
                )),
                // just create
                1 => self.create(name, None),
                _ => None,
            };
            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            return rtn;
        }
        None
    }

    // Delete the venv directory
    pub fn delete(&self, name: String) -> Option<String> {
        // enter alt screen
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
        )
        .unwrap();

        let menu = vec![
            interactive::MenuItem {
                text: "Yes".to_string(),
            },
            interactive::MenuItem {
                text: "No".to_string(),
            },
        ];

        // make a new menu
        let mut menu = interactive::Menu::new(
            format!("Are you sure you want to delete {}?", name),
            menu,
            false,
        );

        // ask the user to select the venv from the menu
        if let Some(choice) = menu.display() {
            if choice as usize > menu.menu_items.len() {
                return None;
            }

            // return the env to activate
            let cmd = format!("rm -rf {}/{}", self.venv_store.to_str().unwrap(), name);

            // create the command to delete the folder holding the venv
            return Some(cmd);
        }
        None
    }

    pub fn delete_interactive(&self) -> Option<String> {
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
        let mut menu = interactive::Menu::new("Choose a venv".to_string(), menu, true);
        //     {
        //     prompt: "Choose a venv".to_string(),
        //     cursor_pos: 0,
        //     menu_items: menu,
        //     searchable: true,
        // };

        // ask the user to select the venv from the menu
        if let Some(choice) = menu.display() {
            if choice as usize > menu.menu_items.len() {
                return None;
            }

            // return the env to delete
            let cmd = self.delete(menu.menu_items[choice as usize].text.clone());

            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            // create the command to delete the folder holding the venv
            return cmd;
        }
        None
    }

    fn get_venv_vec(&self) -> Vec<MenuItem> {
        // put the available venv's in a menu
        let mut menu = Vec::new();
        let envs = fs::read_dir(&self.venv_store).unwrap();
        for f in envs {
            let name = f.unwrap().file_name();
            let name = name.to_str().unwrap().to_string();
            if name != ".history" && name != "bin" {
                menu.push(interactive::MenuItem { text: name })
            }
        }

        // alphabetical order
        menu.sort_by_key(|i| i.text.clone());
        menu
    }

    pub fn list(&self) {
        let a = "Available venvs:";
        eprintln!("{}", a.blue());
        let d = self.get_venv_vec();
        for v in d.iter() {
            let fmt = format!("{}", v.text);
            eprintln!("  {}", fmt.yellow());
        }
    }

    fn get_py_versions(&self) -> Vec<PyVersion> {
        let mut dedup: Vec<String> = vec![];
        let versions: Vec<PyVersion> = [
            "python3",
            "python3.13",
            "python3.12",
            "python3.11",
            "python3.10",
            "python3.9",
        ]
        .iter()
        .map(|e| {
            let pythons = process::Command::new("which").args(["-a", e]).output();
            let cmd_out = String::from_utf8(pythons.unwrap().stdout).unwrap();
            let pys: Vec<PyVersion> = cmd_out
                .lines()
                .filter_map(|pth| {
                    let ver = process::Command::new(pth).args(["-V"]).output();
                    let ver_string = String::from_utf8(ver.unwrap().stdout).unwrap();
                    if ver_string.is_empty() {
                        return None;
                    }
                    Some(
                        ver_string
                            .lines()
                            .filter_map(|e| {
                                let v = e.split_whitespace().nth(1).unwrap();
                                if dedup.contains(&v.to_string()) {
                                    return None;
                                }
                                dedup.push(v.to_string());
                                Some(PyVersion(v.to_string(), pth.to_string()))
                            })
                            .collect::<Vec<PyVersion>>(),
                    )
                })
                .flatten()
                .collect();
            pys
        })
        .flatten()
        .collect();

        versions
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_get_py_versions() {
        let venv = VenvManager::new().unwrap();
        let pys = venv.get_py_versions();
        for v in pys {
            println!("{}: {}", v.0, v.1);
        }
    }
}
