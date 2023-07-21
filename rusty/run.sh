#!/bin/zsh
binName='venv'
rm $binName

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/$binName .
	shift
else
	cargo build &&
		mv target/debug/$binName .
fi

if [[ $? -ne 0 ]]; then
	exit 1
fi

clear

./$binName $@
rm $binName
cat ~/.venv/history
