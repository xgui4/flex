#!/usr/bin/env sh

if [ "$1" = "debian" ]; then
    echo "Coming Soon!"
fi

if [ "$1" = "appimage" ]; then
    echo "Coming Soon!"
fi

if [ "$1" = "flatpak" ]; then
    echo "Coming Soon!"
fi

if [ "$1" = "windows" ]; then
    echo "Coming Soon"
fi

if [ "$1" = "aur" ]; then 
    cd "packages/linux/nightly" || return 0
    makepkg --si
fi

if [ "$1" = "aur-stable" ]; then 
    echo "This is for the 0.0.2.0 stable version who is might not be available yet"
    cd "packages/linux/stable" || return 0
    makepkg --si
fi