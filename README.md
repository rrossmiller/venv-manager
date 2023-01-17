# venv-manager

![](example.gif)

## Install

```
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/bin/install.sh | /bin/zsh
```

Copy the function that it prints out into your ~/.zhsrc or bashrc

```
function venv(){
    govenv $@
    if [[ $? -eq 0 ]]; then
        eval `tail -n 1 ~/.venv/history`
    fi
}
```

```
source ~/.zshrc #(or bash)
venv
```
