#! /bin/zsh

VERSION=0.1.2
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
echo "Version: $VERSION"
arch=$(uname -m)
os=$(uname)
echo "Downloading venv command for: $os $arch"

ok=0

# check if it's and M1 mac
if [[ $os == "Darwin" && $arch == "arm64" ]]; then
    appleSilicon="y"
elif [[ $os == "Darwin" && $arch != "arm64" ]]; then
    appleSilicon="n"
fi

if [[ $appleSilicon == "y" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/$VERSION/govenv-darwin-arm64
    ok=1

elif [[ $appleSilicon = "n" ]]; then
    curl -sLJo govenv \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/$VERSION/govenv-darwin-amd64
    ok=1

else
    echo "no windows support"
    exit 1
    # curl -sLJo govenv \
    #     -H "Accept: application/octet-stream" \
    #     https://github.com/rrossmiller/venv-manager/releases/download/$VERSION/govenv-windows-amd64
    # ok=1

fi

if [[ ok -eq 1 ]]; then
    chmod +x govenv
    sudo mv govenv /usr/local/bin
    echo
    echo "add this to your .bashrc or .zshrc"
    echo '
function venv(){
    govenv $@
    if [[ $? -eq 0 ]]; then
        eval `tail -n 1 ~/.venv/history`
    fi
}'
else
    echo "something went wrong"
    echo "ok=$ok, appleSilicon=$appleSilicon"
fi
