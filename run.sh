cd src
rm govenv
clear
go build &&
	./govenv -i $@

v=$(tail -n 1 ~/.venv/history)
echo
cd ..
eval $v
# echo $out
# cd ..
# eval $out
which python
