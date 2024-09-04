#! /bin/zsh

VERSION=0.5.0
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
    curl -sLJo venv_manager \
        -H "Accept: application/octet-stream" \
        https://github.com/rrossmiller/venv-manager/releases/download/$VERSION/rs-venv-darwin-arm64
    ok=1

# build from scratch
elif [[ $appleSilicon = "n" ]]; then
    if type cargo &>/dev/null; then
	pth=`pwd`
	echo "building bin"
	git clone https://github.com/rrossmiller/venv-manager/
	cd venv-manager/src/
	./build.sh d
	cd $pth
	rm -rf venv-manager
    else
	echo "Cargo must be installed to build"
	echo "Directions: https://www.rust-lang.org/tools/install"
    fi
    exit 0 
else
    echo "no windows support yet"
    exit 1
fi

if [[ ok -eq 1 ]]; then
    if [[ ! -d "~/.venv/bin" ]]; then
        mkdir -p ~/.venv/bin
    fi

    chmod +x venv_manager
    mv venv_manager ~/.venv/bin
    echo
    echo "add this to your .bashrc or .zshrc"
    echo '
function venv() {
    ~/.venv/bin/venv_manager $@
    if [[ $? -eq 0 ]]; then
        eval $( tail -n 1 ~/.venv/.history )
    fi
}
'
else
    echo "something went wrong"
    echo "ok=$ok, appleSilicon=$appleSilicon"
fi
