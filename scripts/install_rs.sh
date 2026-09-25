#! /bin/bash

VERSION=0.7.0
venv_path="$HOME/.venvs"
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
        pth=$(pwd)
        echo "building bin"
        git clone https://github.com/rrossmiller/venv-manager/
        cd venv-manager/
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

if [[ $ok -eq 1 ]]; then
    # move the bin to the venv_path
    if [[ ! -d $venv_path/bin ]]; then
        mkdir -p $venv_path/bin
    fi

    chmod +x venv_manager
    mv venv_manager $venv_path/bin

    # create the venv function if it doesn't exist

    shell_name=$(basename "$SHELL")
    function_definition="
function venv() {
    \"$venv_path/bin/venv_manager\" \"\$@\"
    if [[ \$? -eq 0 ]]; then
        eval \$( tail -n 1 $venv_path/.history )
    fi
}
"
    if [[ "$shell_name" == "bash" ]]; then
        shell_config="$HOME/.bashrc"
    elif [[ "$shell_name" == "zsh" ]]; then
        shell_config="$HOME/.zshrc"
    else
        echo "Unsupported shell: $shell_name. Please add the function manually."
        exit 0
    fi

    if ! grep -q "function venv" "$shell_config"; then
        echo "$function_definition" >>"$shell_config"
        echo "Function 'venv' added to $shell_config"
    fi

    completion_definition="source <(COMPLETE=$shell_name \"$venv_path/bin/venv_manager\")"
    if ! grep -Fq "$completion_definition" "$shell_config"; then
        echo "$completion_definition" >>"$shell_config"
        echo "Tab completion for 'venv' added to $shell_config"
    fi

    echo
    echo "Open a new terminal or \`source\` your ~/.bashrc or ~/.zshrc"

else
    echo "something went wrong"
    echo "ok=$ok, appleSilicon=$appleSilicon"
fi
