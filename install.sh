#! /bin/zsh
cd src
go build
sudo mv govenv /usr/local/bin

echo "add this to your .bashrc or .zshrc"
echo "
function venv(){
    clear
    govenv $@
    if [[ $? -eq 0 ]]; then
        eval \`tail -n 1 ~/.venv/history\`
        clear
    fi
}
"
