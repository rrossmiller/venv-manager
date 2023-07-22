#!/bin/zsh
binName='venv'
outName="${binName}_manager"
rm $outName

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/$binName $outName
	exit 0
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
		mv target/release/$binName $outName
else
	cargo build &&
		mv target/debug/$binName $outName
	exit 0
fi

mv $outName /usr/local/bin
