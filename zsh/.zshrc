##-------------------- exports --------------------

export PATH="$PATH:$HOME/.cargo/bin:$HOME/.dotnet/tools/:$HOME/.local/bin:$HOME/.config/go/bin"
GOPATH=".config/go"

##-------------------- history --------------------

export HISTSIZE=100000000
export SAVEHIST=$HISTSIZE
export HISTFILE=$HOME/.local/zsh_history

##-------------------- prompt --------------------

autoload -U colors && colors
LS_COLORS=$LS_COLORS:"di=1;95:" ; export LS_COLORS
PS1="%B%{$fg[red]%}[%{$fg[yellow]%}%n%{$fg[green]%}@%{$fg[blue]%}%M %{$fg[magenta]%}%~%{$fg[red]%}]%{$reset_color%}$%b "

##-------------------- aliases --------------------

alias ls="eza --group-directories-first --git -h"
alias grep="grep --color=auto"

bindkey "^[[1;3C" forward-word
bindkey "^[[1;3D" backward-word

##-------------------- syntax highlighting --------------------

ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#698b69,bg=#1d2021,bold"

source ~/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh
source ~/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
