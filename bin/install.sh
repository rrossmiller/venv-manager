#! /bin/bash
cd src
go build
sudo mv govenv /usr/local/bin
#! /bin/bash
read -p "Is your computer using Mac Silicon? (y/n)" macSilicon
ok=false

if [[ $macSilicon = "y" ]]; then
    curl -o 
elif [[ $macSilicon = "n" ]]; then
    echo "n"
fi

if [[ ok ]]; then

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

fi
