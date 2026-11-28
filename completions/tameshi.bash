# bash completion for tameshi
_tameshi_completions() {
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    if [[ "$prev" == "--ruling" ]]; then
        COMPREPLY=( $(compgen -W "keep demote quarantine kill" -- "$cur") )
        return 0
    fi
    if [[ "$cur" == -* ]] || [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "assemble rule validate validate-ruling --catchledger --canaryfield --blindgate --decision --packs-dir --pack --ruling --expires --today --out --body -h --help" -- "$cur") )
        return 0
    fi
    COMPREPLY=( $(compgen -f -- "$cur") )
}
complete -F _tameshi_completions tameshi
