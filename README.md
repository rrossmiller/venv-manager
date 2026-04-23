# venv-manager

![](example.gif)

## Install

```sh
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/scripts/install_rs.sh | /bin/zsh
```

The installer adds a `venv` shell function to `~/.zshrc` or `~/.bashrc`. To add it manually, use:

```sh
function venv() {
    ~/.venvs/bin/venv_manager "$@"
    if [[ $? -eq 0 ]]; then
        eval "$(tail -n 1 ~/.venvs/.history)"
    fi
}
```

Then reload your shell:

```sh
source ~/.zshrc # or ~/.bashrc
venv
```

## Usage

```sh
venv list
venv create my-project 3.12
venv activate my-project
venv delete my-project
venv my-project
```

Running `venv` with no arguments opens the interactive picker.
