use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn completion_home() -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("venv-completion-test-{unique}"))
}

#[test]
fn zsh_completion_includes_matching_venvs_and_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let test_home = completion_home();
    let venv_store = test_home.join(".venvs");
    fs::create_dir_all(venv_store.join("api"))?;
    fs::create_dir_all(venv_store.join("analytics"))?;
    fs::write(venv_store.join("api/pyvenv.cfg"), "version = 3.12.7\n")?;
    fs::write(
        venv_store.join("analytics/pyvenv.cfg"),
        "version = 3.11.10\n",
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_venv"))
        .args(["--", "venv", ""])
        .env("HOME", &test_home)
        .env("COMPLETE", "zsh")
        .env("_CLAP_COMPLETE_INDEX", "1")
        .env("_CLAP_IFS", "\n")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("activate:Activate a virtual environment"));
    assert!(stdout.contains(&format!(
        "analytics:Python 3.11.10 — {}",
        venv_store.join("analytics").display()
    )));
    assert!(stdout.contains(&format!(
        "api:Python 3.12.7 — {}",
        venv_store.join("api").display()
    )));
    Ok(())
}

#[test]
fn activate_completion_only_includes_matching_venvs() -> Result<(), Box<dyn std::error::Error>> {
    let test_home = completion_home();
    let venv_store = test_home.join(".venvs");
    fs::create_dir_all(venv_store.join("api"))?;
    fs::create_dir_all(venv_store.join("web"))?;

    let output = Command::new(env!("CARGO_BIN_EXE_venv"))
        .args(["--", "venv", "activate", "a"])
        .env("HOME", &test_home)
        .env("COMPLETE", "zsh")
        .env("_CLAP_COMPLETE_INDEX", "2")
        .env("_CLAP_IFS", "\n")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains(&format!("api:{}", venv_store.join("api").display())));
    assert!(!stdout.contains("web"));
    assert!(!stdout.contains("create:"));
    Ok(())
}
