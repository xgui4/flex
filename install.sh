#!/usr/bin/env sh

if [ "$1" = "debian" ]; then
    echo "Coming Soon!"
fi

if [ "$1" = "freebsd" ]; then
    echo "Coming Soon!"
fi

if [ "$1" =  "appimage" ]; then
    echo "Coming Soon!"
fi

if [ "$1" = "windows" ]; then
    echo "Coming Soon"
fi

if [ "$1" = "aur" ]; then 
    cd "packages/linux" || return 0
    makepkg -p PKGBUILD-git --install
fi

if [ "$1" = "aur-stable" ]; then 
    echo "This is for the 0.0.2.0 stable version who is might not be available yet"
    cd "packages/linux" || return 0
    makepkg --install
fi