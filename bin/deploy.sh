clear
cd ../src
echo AMD64
GOOS=darwin GOARCH=amd64 go build -o ../bin/govenv-darwin-amd64

echo ARCH64
GOOS=darwin GOARCH=arm64 go build -o ../bin/govenv-darwin-arm64

VERSION=0.1.2
echo "Update version in install.sh to $VERSION"
echo
gh release create --generate-notes $VERSION ../bin/govenv-darwin-arm64 ../bin/govenv-darwin-amd64
