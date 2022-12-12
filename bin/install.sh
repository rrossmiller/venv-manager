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
read -q "?Is your computer using Mac Silicon? (y/n) " macSilicon
ok=false

if [[ $macSilicon == "y" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-arm64
    ok=true
elif [[ $macSilicon = "n" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/0.0.1/govenv-amd64
    ok=false
fi

if [[ ok ]]; then
    chmod +x govenv
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
else
    echo "something went wrong"
    echo "ok=$ok, macSilicon=$macSilicon"
fi
