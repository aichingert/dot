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

autoload -U colors && colors
LS_COLORS=$LS_COLORS:"di=1;95:" ; export LS_COLORS
PS1="%F{#e8b589}%n%F{#6e94b2}[%~] %F{#aeaed1}%T%F{#df6882}%(?.. [%?])%F{#aeaed1}>"

##-------------------- aliases --------------------

alias ls="eza --group-directories-first --git -h --icons=always"
alias grep="grep --color=auto"

bindkey "^[[1;3C" forward-word
bindkey "^[[1;3D" backward-word

##-------------------- syntax highlighting --------------------
# bg : 1d2021 | 252530 | 

ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#606079,bg=#141415,bold"

source ~/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh
source ~/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

