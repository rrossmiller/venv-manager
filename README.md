# venv-manager

![](example.gif)

## Install
```
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/scripts/install_rs.sh | /bin/zsh
```


Copy the function into ~/.zhsrc or ~/.bashrc

```
function venv() {
	source ~/.venv/venv_manager $@
	if [[ $? -eq 0 ]]; then
		eval $(tail -n 1 ~/.venv/.history)
	fi
}

```

```
source ~/.zshrc #(or ~/.bashrc)
venv
```

Go version (deprecated):

```
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/scripts/install_go.sh | /bin/zsh

```
