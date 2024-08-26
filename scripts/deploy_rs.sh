VERSION=0.5.0
if [[ $1 == 'r' ]]; then
    clear
    bin="rs-venv-darwin-arm64"
    cd ../
    rm $bin

    echo "AMD64"
    cargo build --release && 
        mv target/release/venv $bin

    echo "Update version in install.sh to $VERSION"
    echo
    gh release create --generate-notes $VERSION $bin
    rm $bin

elif [[ $1 == 'd' ]]; then
# to delete:
    gh release delete $VERSION; git push --delete origin $VERSION
else
    echo "must enter 'r' for release or 'd' for delete"
fi
