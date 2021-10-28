#!/bin/bash

cp tailwin.desktop /usr/share/xsessions
cp tialwin.png /usr/share/pixmaps
cargo bulid --release
ln -f /target/release/tailwin /usr/bin/tailwin
