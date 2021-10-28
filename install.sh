#!/bin/bash

echo "---cp tailwin.desktop /usr/share/xsessions---"
sudo cp tailwin.desktop /usr/share/xsessions
echo "---cp tailwin.png /usr/share/pixmaps---"
sudo cp tailwin.png /usr/share/pixmaps
echo "---cargo build --release---"
cargo build --release
echo "---ln -f ./target/release/tailwin /usr/bin/tailwin---"
sudo ln -f ./target/release/Tailwin /usr/bin/tailwin
