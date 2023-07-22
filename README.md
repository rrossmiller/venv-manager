# venv-manager

![](example.gif)

## Install
🚧 I'm working on an automated install script for the newer rust version...

for now, you can run the following on Linux or Mac
```
cd rust_src; ./build.sh d
```

Go version (deprecated):

```
curl -s https://raw.githubusercontent.com/rrossmiller/venv-manager/main/scripts/install.sh | /bin/zsh

```

Copy the function into ~/.zhsrc or ~/.bashrc

```
function venv() {
	venv_manager $@
	if [[ $? -eq 0 ]]; then
		eval $(tail -n 1 ~/.venv/.history)
	fi
}

```

```
source ~/.zshrc #(or ~/.bashrc)
venv
```
