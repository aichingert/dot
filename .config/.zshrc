##-------------------- good to know ---------------
#
# pactl set-sink-volume @DEFAULT_SINK@ +5%
#
##-------------------- exports --------------------

export PATH="$PATH:$HOME/.cargo/bin:$HOME/.dotnet/tools/:$HOME/.local/bin:$HOME/.config/go/bin"
GOPATH=".config/go"

##-------------------- history --------------------

export HISTSIZE=100000000
export SAVEHIST=$HISTSIZE
export HISTFILE=$HOME/.local/zsh_history

##-------------------- prompt --------------------

# TODO: set this depending on the theme
PS1="%F{#e8b589}%n%F{#6e94b2}[%~] %F{#aeaed1}%T%F{#df6882}%(?.. [%?])%F{#aeaed1}> "

##-------------------- aliases --------------------

alias ga="git add"
alias gd="git diff"
alias gs="git status --short"
alias gp="git push"
alias gu="git pull"
alias gr="git reset --staged"
# | %h -- commit hash 
# | %an -- author name 
# | %ar -- commit time k
# | %D -- ref names 
# | %n -- new line 
# | %s -- commit message
alias gl="git log --all --graph --pretty=\
    format:'%C(magenta)%h %C(white) %an %ar%C(auto) %D%n%s%n'"
alias gc="git commit"
alias gcl="git clone"
alias pacup="pacman -Syu"
alias pacrm="pacman -R"

alias ls="eza --group-directories-first --git -h --icons=always"
alias grep="grep --color=auto"


zmodload zsh/terminfo
bindkey "^[[1;3C" forward-word
bindkey "^[[1;3D" backward-word

##-------------------- syntax highlighting --------------------
# bg : 1d2021 | 252530 | 

ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#606079,bg=#141415,bold"

ZSH=~/.zsh

source $ZSH/zsh-users/zsh-autosuggestions/zsh-autosuggestions.zsh
source $ZSH/zsh-users/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

