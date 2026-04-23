# venv-manager

![](example.gif)

## Install
```sh
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/scripts/install_rs.sh | /bin/zsh
```

The installer adds the `venv` shell function and tab completion for bash or zsh.

After reloading your shell, `venv a<Tab>` will complete to a matching environment like `api`.

If you want to install it manually, copy the matching setup into `~/.zshrc` or `~/.bashrc`.

```sh
function venv() {
    ~/.venvs/bin/venv_manager "$@"
    if [[ $? -eq 0 ]]; then
        eval "$(tail -n 1 ~/.venvs/.history)"
    fi
}
 
_venv_complete() {
    local -a envs
    envs=(${(f)"$(~/.venvs/bin/venv_manager list-names 2>/dev/null)"})
    if (( CURRENT == 2 )) || [[ "$words[CURRENT-1]" == "activate" ]] || [[ "$words[CURRENT-1]" == "delete" ]]; then
        compadd -- $envs
    fi
}
 
compdef _venv_complete venv
```

For bash:

```sh
function venv() {
    ~/.venvs/bin/venv_manager "$@"
    if [[ $? -eq 0 ]]; then
        eval "$(tail -n 1 ~/.venvs/.history)"
    fi
}

_venv_complete() {
    local current prev
    COMPREPLY=()
    current="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    if [[ $COMP_CWORD -eq 1 || "$prev" == "activate" || "$prev" == "delete" ]]; then
        local envs
        envs=$(~/.venvs/bin/venv_manager list-names 2>/dev/null)
        COMPREPLY=($(compgen -W "$envs" -- "$current"))
    fi
}

complete -F _venv_complete venv
```

Then reload your shell:

```sh
source ~/.zshrc
# or
source ~/.bashrc
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
