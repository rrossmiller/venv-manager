use home;
use interactive::MenuItem;
use std::{
    fs,
    io::{self, Write},
    path::{self, PathBuf},
    process,
};

use crossterm::{cursor, execute, style::Stylize, terminal};
mod interactive;

const VENV_STORE: &str = ".venvs";
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
            fs::create_dir(home_dir.as_path()).expect("Error creating new .venvs dir");
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

        let mut menu = interactive::Menu::new("Choose an option".to_string(), menu, false);
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
        let activate_path = self.venv_path(&name).join("bin").join("activate");
        Some(format!("source {}", shell_escape_path(&activate_path)))
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
            if choice >= menu.menu_items.len() {
                return None;
            }
            // return the env to activate
            return self.activate(menu.menu_items[choice].text.clone());
        }
        None
    }

    /// Create a new venv from the user's input name
    pub fn create(&self, name: String, version: Option<String>) -> Option<String> {
        eprintln!("Creating venv '{name}'");
        Some(self.create_with_python(name.as_str(), normalize_python_command(version.as_deref())))
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
        if versions.is_empty() {
            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            eprintln!("No Python interpreters found");
            return None;
        }
        let menu = versions
            .iter()
            .map(|v| interactive::MenuItem { text: v.0.clone() })
            .collect();
        let mut menu = interactive::Menu::new("Choose a Python version".to_string(), menu, false);
        let idx = menu.display()?;
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
                0 => Some(self.create_and_activate_with_python(name.as_str(), py_ver.1.as_str())),
                // just create
                1 => Some(self.create_with_python(name.as_str(), py_ver.1.as_str().to_string())),
                _ => None,
            };
            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            return rtn;
        }
        execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        None
    }

    // Delete the venv directory
    pub fn delete(&self, name: String) -> Option<String> {
        Some(format!(
            "rm -rf -- {}",
            shell_escape_path(&self.venv_path(&name))
        ))
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
            if choice >= menu.menu_items.len() {
                return None;
            }

            let name = menu.menu_items[choice].text.clone();
            let cmd = self.confirm_delete(name);

            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
            // create the command to delete the folder holding the venv
            return cmd;
        }
        execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
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
                    let ver = ver.unwrap();
                    let ver_string = if ver.stdout.is_empty() {
                        String::from_utf8(ver.stderr).unwrap()
                    } else {
                        String::from_utf8(ver.stdout).unwrap()
                    };
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

    fn confirm_delete(&self, name: String) -> Option<String> {
        let menu = vec![
            interactive::MenuItem {
                text: "Yes".to_string(),
            },
            interactive::MenuItem {
                text: "No".to_string(),
            },
        ];

        let mut menu =
            interactive::Menu::new(format!("Delete virtual environment '{name}'?"), menu, false);

        match menu.display() {
            Some(0) => self.delete(name),
            _ => None,
        }
    }

    fn venv_path(&self, name: &str) -> PathBuf {
        self.venv_store.join(name)
    }

    fn create_with_python(&self, name: &str, python_cmd: String) -> String {
        format!(
            "{} -m venv {}",
            shell_escape(&python_cmd),
            shell_escape_path(&self.venv_path(name))
        )
    }

    fn create_and_activate_with_python(&self, name: &str, python_cmd: &str) -> String {
        let venv_path = self.venv_path(name);
        let activate_path = venv_path.join("bin").join("activate");
        format!(
            "{} -m venv {} && source {}",
            shell_escape(python_cmd),
            shell_escape_path(&venv_path),
            shell_escape_path(&activate_path)
        )
    }
}

fn normalize_python_command(version: Option<&str>) -> String {
    match version {
        Some(version) if version.starts_with("python") => version.to_string(),
        Some(version) => format!("python{version}"),
        None => "python3".to_string(),
    }
}

fn shell_escape_path(path: &path::Path) -> String {
    shell_escape(path.to_string_lossy().as_ref())
}

fn shell_escape(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_manager() -> VenvManager {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        VenvManager {
            venv_store: std::env::temp_dir().join(format!("venv-manager-{unique}")),
        }
    }

    #[test]
    fn activate_escapes_generated_source_path() {
        let manager = VenvManager {
            venv_store: PathBuf::from("/tmp/venvs with spaces"),
        };

        let cmd = manager.activate("demo'env".to_string()).unwrap();

        assert_eq!(
            cmd,
            "source '/tmp/venvs with spaces/demo'\"'\"'env/bin/activate'"
        );
    }

    #[test]
    fn create_uses_requested_python_version() {
        let manager = test_manager();

        let cmd = manager
            .create("demo".to_string(), Some("3.12".to_string()))
            .unwrap();

        assert!(cmd.starts_with("'python3.12' -m venv "));
    }

    #[test]
    fn create_accepts_full_python_command() {
        assert_eq!(
            normalize_python_command(Some("python3.11")),
            "python3.11".to_string()
        );
    }

    #[test]
    fn delete_uses_safe_rm_command() {
        let manager = test_manager();

        let cmd = manager.delete("demo".to_string()).unwrap();

        assert!(cmd.starts_with("rm -rf -- '"));
        assert!(cmd.contains("/demo'"));
    }

    #[test]
    fn test_get_py_versions() {
        let venv = VenvManager::new().unwrap();
        let pys = venv.get_py_versions();
        for v in pys {
            println!("{}: {}", v.0, v.1);
        }
    }
}
