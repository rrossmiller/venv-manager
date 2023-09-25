#!/bin/zsh
binName='venv'
outName="${binName}_manager"
rm $outName

# release build and mv to current dir
if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/$binName $outName
	exit 0

# build and move to current dir (then move to /usr/local/bin)
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
	    mv target/release/$binName $outName &&
        sudo mv $outName /usr/local/bin

# build and move to current dir
else
	cargo build &&
		mv target/debug/$binName $outName
	exit 0
fi

