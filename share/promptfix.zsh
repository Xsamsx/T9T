typeset -g PROMPTFIX_BIN="${PROMPTFIX_BIN:-promptfix}"
typeset -g PROMPTFIX_LAST_HINT=""

_promptfix_run_check() {
  local buffer="$1"
  [[ -z "$buffer" ]] && return 0
  "$PROMPTFIX_BIN" check --text "$buffer" 2>/dev/null
}

_promptfix_show_hints() {
  local output="$1"
  local hints=()
  local line original replacement

  while IFS= read -r line; do
    case "$line" in
      MESSAGE\ *)
        original="${${(z)line}[2]}"
        replacement="${${(z)line}[3]}"
        hints+=("${original} -> ${replacement}")
        ;;
    esac
  done <<< "$output"

  if (( ${#hints[@]} > 0 )); then
    local joined="${(j:, :)hints}"
    PROMPTFIX_LAST_HINT="$joined"
    zle -M -- "$joined"
  else
    PROMPTFIX_LAST_HINT=""
  fi
}

_promptfix_apply_buffer() {
  local output="$1"
  local line

  while IFS= read -r line; do
    case "$line" in
      APPLY\ *)
        BUFFER="${line#APPLY }"
        CURSOR=${#BUFFER}
        return 0
        ;;
    esac
  done <<< "$output"

  return 1
}

_promptfix_space() {
  zle self-insert
  local output
  output="$(_promptfix_run_check "$BUFFER")"
  _promptfix_show_hints "$output"
}

_promptfix_tab() {
  local output
  output="$(_promptfix_run_check "$BUFFER")"
  if [[ -n "$output" ]] && _promptfix_apply_buffer "$output"; then
    zle redisplay
    _promptfix_show_hints "$output"
  else
    zle expand-or-complete
  fi
}

_promptfix_enter() {
  local output
  output="$(_promptfix_run_check "$BUFFER")"
  _promptfix_show_hints "$output"
  zle accept-line
}

zle -N promptfix-space _promptfix_space
zle -N promptfix-tab _promptfix_tab
zle -N promptfix-enter _promptfix_enter

bindkey ' ' promptfix-space
bindkey '^I' promptfix-tab
bindkey '^M' promptfix-enter
