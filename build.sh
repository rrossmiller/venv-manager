#!/bin/zsh
binName='venv'
outName="${binName}_manager"
rm $outName

# release build and mv to current dir
if [[ $1 == 'r' ]]; then
    echo "building release"
    cargo build --release &&
    mv target/release/$binName $outName

# build and move to current dir (then move to bin dir)
elif [[ $1 == 'd' ]]; then
    echo "deploy release"
    if [[ ! -d "~/.venvs/bin" ]]; then
        mkdir -p ~/.venvs/bin
    fi
    cargo build --release &&
    mv target/release/$binName ~/.venvs/bin/$outName 

# build and move to current dir
else
    cargo build &&
    mv target/debug/$binName $outName
fi

