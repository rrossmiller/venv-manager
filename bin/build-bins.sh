clear
cd ../src
echo AMD64
GOOS=darwin	GOARCH=amd64 go build -o ../bin/govenv-darwin-amd64

echo ARCH64
GOOS=darwin	GOARCH=arm64 go build -o ../bin/govenv-darwin-arm64

# echo Windows
# GOOS=windows GOARCH=amd64 go build -o ../bin/govenv-windows-amd64.exe