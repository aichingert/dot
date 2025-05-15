#!/bin/sh

set -e

DOT=~/git/dot
ZSH=~/.zsh/
CONF=~/.config

mkdir -p $ZSH
mkdir -p $CONF

type zsh >/dev/null 2>&1 || {
    echo "installing: zsh..."
    sudo pacman -S zsh
    chsh -s zsh
}

type alacritty >/dev/null 2>&1 || {
    echo "installing: alacritty..."
    sudo pacman -S alacritty
}

type nvim >/dev/null 2>&1 || {
    echo "installing: neovim..."
    sudo pacman -S neovim
}

zsh_plugins=(zsh-users/zsh-autosuggestions zsh-users/zsh-syntax-highlighting)

for plugin in ${zsh_plugins[@]}
do
    if [ ! -d $ZSH$plugin ] ; then
        echo "cloning: $plugin..."
        git clone git@github.com:$plugin.git --depth=1 -q $ZSH$plugin
    fi
done

cp -r $DOT/.config/nvim $CONF
cp -r $DOT/.config/alacritty $CONF
cp -r $DOT/.config/.zshrc ~/.zshrc

zsh -i -c "source ~/.zshrc"
