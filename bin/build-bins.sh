clear
cd ../src
echo AMD64
GOOS=darwin	GOARCH=amd64 go build -o ../bin/govenv-darwin-amd64

echo ARCH64
GOOS=darwin	GOARCH=arm64 go build -o ../bin/govenv-darwin-arm64
