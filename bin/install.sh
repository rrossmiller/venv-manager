#! /bin/bash
git clone https://github.com/rrossmiller/venv-manager.git ~/.venv-manager && \
cd ~/.venv-manager/src
go build && \
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

# #! /bin/bash
# read -p "Is your computer using Mac Silicon? (y/n) " macSilicon
# ok=false

# if [[ $macSilicon = "y" ]]; then
#     curl -o govenv https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-arm64
#     ok=true
# elif [[ $macSilicon = "n" ]]; then
#     curl -o govenv https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-amd64
#     ok=true
# fi

# if [[ ok ]]; then
#     chmod +x govenv
#     # sudo mv govenv /usr/local/bin
#     echo "add this to your .bashrc or .zshrc"
#     echo "
# function venv(){
#     clear
#     govenv $@
#     if [[ $? -eq 0 ]]; then
#         eval \`tail -n 1 ~/.venv/history\`
#         clear
#     fi
# }
# "
# else
#     echo "something went wrong"
#     echo "ok=$ok, macSilicon=$macSilicon"
# fi
