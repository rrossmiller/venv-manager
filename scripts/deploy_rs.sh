VERSION=0.2.0
if [[ $1 == 'r' ]]; then
    clear
    bin="rs-venv-darwin-arm64"
    cd ../rust_src
    rm $bin

    echo "AMD64"
    cargo build --release && 
        mv target/release/venv $bin

    echo "Update version in install.sh to $VERSION"
    echo
    gh release create --generate-notes $VERSION $bin

elif [[ $1 == 'd' ]]; then
# to delete:
    gh release delete $VERSION; git push --delete origin $VERSION
fi