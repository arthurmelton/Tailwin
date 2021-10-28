#!/bin/bash

echo "---cp tailwin.desktop /usr/share/xsessions---"
sudo cp tailwin.desktop /usr/share/xsessions
echo "---cp tailwin.png /usr/share/pixmaps---"
sudo cp tailwin.png /usr/share/pixmaps
echo "---if file exists---"
FILE=./src/tailwin.rs
if [ -f "$FILE" ]; then
    echo "---cargo build --release---"
    cargo build --release
    echo "---ln -f ./target/release/tailwin /usr/bin/tailwin---"
    sudo ln -f ./target/release/tailwin /usr/bin/tailwin
else 
    echo "---mkdir ~/.config/tailwin---"
    mkdir ~/.config/tailwin
    echo "---printf 'fn on_startup() {\n\t// Do stuff\n}\n\npub fn on_key(key:u32) {\n\t// Do stuff\n}' > ~/.config/tailwin/tailwin.rs---"
    printf 'fn on_startup() {\n\t// Do stuff\n}\n\npub fn on_key(key:u32) {\n\t// Do stuff\n}' > ~/.config/tailwin/tailwin.rs
    echo "---ln -f ~/.config/tailwin/tailwin.rs ./src/tailwin.rs---"
    ln -f ~/.config/tailwin/tailwin.rs ./src/tailwin.rs
    echo ""
    echo "Edit your config file in ~/.config/tailwin/tailwin.rs and then run the install script again"
fi
