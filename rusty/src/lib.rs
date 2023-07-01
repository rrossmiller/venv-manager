use colored::Colorize;
use home;
use std::{fs, path};

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

    pub fn activate() {
        todo!();
    }
    pub fn create() {
        todo!();
    }
    pub fn list(&self) {
        let a = "Available venvs:";
        println!("{}", a.blue());
        let d = fs::read_dir(&self.venv_store).unwrap();
        for f in d {
            let fmt = format!("{}", f.unwrap().file_name().to_str().unwrap());
            println!("  {}", fmt.yellow());
        }
    }

    pub fn delete() {
        todo!();
    }
}
