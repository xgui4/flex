if (( $EUID != 0 )); then
    echo "Please run as root or with sudo."
    exit 1
fi

wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /usr/local/bin/appimagetool
chmod +x /usr/local/bin/appimagetool

cargo install cargo-appimage