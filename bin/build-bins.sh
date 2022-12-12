clear
cd ../src
GOOS=darwin	GOOARCH=amd64 go build && mv govenv ../bin/govenv-amd64
GOOS=darwin	GOOARCH=arm64 go build && mv govenv ../bin/govenv-arm64