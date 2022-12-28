clear
cd ../src
echo AMD64
GOOS=darwin GOARCH=amd64 go build -o ../bin/govenv-darwin-amd64

echo ARCH64
GOOS=darwin GOARCH=arm64 go build -o ../bin/govenv-darwin-arm64

VERSION=0.1.1
echo "Update version in install.sh to $VERSION"
echo
gh release create
gh release upload $VERSION govenv-darwin-arm64 govenv-darwin-amd64

