if test -f /usr/local/bin/nyancat; then
    echo "nyancat binary already exists, removing for fresh build..."
    sudo rm /usr/local/bin/nyancat
fi

cargo build --release

sudo cp ./target/release/nyancat /usr/local/bin/
