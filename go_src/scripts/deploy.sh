clear
cd ../
echo AMD64
GOOS=darwin GOARCH=amd64 go build -o ../scripts/govenv-darwin-amd64

echo ARCH64
GOOS=darwin GOARCH=arm64 go build -o ../scripts/govenv-darwin-arm64

VERSION=0.1.2
echo "Update version in install.sh to $VERSION"
echo
gh release create --generate-notes $VERSION ../scripts/govenv-darwin-arm64 ../scripts/govenv-darwin-amd64

# to delete: gh release delete $VERSION; git push --delete origin $VERSION
