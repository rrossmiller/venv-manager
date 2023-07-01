#!/bin/zsh
binName='venv'
rm $binName

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/$binName .
	exit 0
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
		mv target/release/$binName .
else
	cargo build &&
		mv target/debug/$binName .
	exit 0
fi

sudo mv $binName /usr/local/bin
