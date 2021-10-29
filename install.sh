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
    echo "---~/.config/tailwin/tailwin.rs---"
    printf 'pub fn on_startup() {\n\t// Do stuff\n}\n\npub fn on_key(key:u32, state:i8) -> String {\n\t// Do stuff\n\t// "destroy" will destroy the curent window\n\treturn "null".to_string();\n}\n\npub fn mod_mask() -> i8 {\n\t// this will set the mod key to the windows key\n\tretrun 4;\n}' > ~/.config/tailwin/tailwin.rs
    echo "---ln -f ~/.config/tailwin/tailwin.rs ./src/tailwin.rs---"
    ln -f ~/.config/tailwin/tailwin.rs ./src/tailwin.rs
    echo ""
    echo "Edit your config file in ~/.config/tailwin/tailwin.rs and then run the install script again"
fi
