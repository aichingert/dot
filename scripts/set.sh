#!/bin/sh

DOT=~/git/dot

function set_theme() {
    if [ ! -d $DOT/themes/$1 ]; then
        echo "[$1 Not Found] Themes: " ; eza -l "$DOT/themes"
        return 
    fi

    cp $DOT/themes/$1/colorscheme.lua ~/.config/nvim/lua/config/colorscheme.lua
    cp $DOT/themes/$1/theme.toml ~/.config/alacritty/theme.toml
}

case $1 in
    ls) echo "Themes: " ; eza -l "$DOT/themes" ;;
    *) set_theme $1 ;;
esac
