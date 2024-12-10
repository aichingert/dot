#!/bin/sh

set -e

DOT=~/git/dot

mkdir -p ~/.config

cp -r $DOT/.config/nvim ~/.config/
cp -r $DOT/.config/alacritty ~/.config/
