#! /bin/zsh
clear
# credit where credit's due: https://patorjk.com/software/taag/#p=display&f=Big%20Money-nw&t=venv%20Manager
echo '
                                               $$\      $$\                                                             
                                               $$$\    $$$ |                                                            
$$\    $$\  $$$$$$\  $$$$$$$\ $$\    $$\       $$$$\  $$$$ | $$$$$$\  $$$$$$$\   $$$$$$\   $$$$$$\   $$$$$$\   $$$$$$\  
\$$\  $$  |$$  __$$\ $$  __$$\\$$\  $$  |      $$\$$\$$ $$ | \____$$\ $$  __$$\  \____$$\ $$  __$$\ $$  __$$\ $$  __$$\ 
 \$$\$$  / $$$$$$$$ |$$ |  $$ |\$$\$$  /       $$ \$$$  $$ | $$$$$$$ |$$ |  $$ | $$$$$$$ |$$ /  $$ |$$$$$$$$ |$$ |  \__|
  \$$$  /  $$   ____|$$ |  $$ | \$$$  /        $$ |\$  /$$ |$$  __$$ |$$ |  $$ |$$  __$$ |$$ |  $$ |$$   ____|$$ |      
   \$  /   \$$$$$$$\ $$ |  $$ |  \$  /         $$ | \_/ $$ |\$$$$$$$ |$$ |  $$ |\$$$$$$$ |\$$$$$$$ |\$$$$$$$\ $$ |      
    \_/     \_______|\__|  \__|   \_/          \__|     \__| \_______|\__|  \__| \_______| \____$$ | \_______|\__|      
                                                                                          $$\   $$ |                    
                                                                                          \$$$$$$  |                    
                                                                                           \______/                     
'
read -q "?Is your computer using Apple Silicon? (y/n) " appleSilicon
ok=0
echo
echo "Downloading venv binary..."
if [[ $appleSilicon == "y" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-darwin-arm64
    ok=1
elif [[ $appleSilicon = "n" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-darwin-amd64
    ok=1
fi

if [[ ok -eq 1 ]]; then
    chmod +x govenv
    sudo mv govenv /usr/local/bin
    echo
    echo "add this to your .bashrc or .zshrc"
    echo "
function venv(){
    clear
    govenv \$@
    if [[ $? -eq 0 ]]; then
        eval \`tail -n 1 ~/.venv/history\`
        clear
    fi
}
"
else
    echo "something went wrong"
    echo "ok=$ok, appleSilicon=$appleSilicon"
fi
