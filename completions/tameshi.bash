# bash completion for tameshi
_tameshi_completions() {
    local cur prev cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd="${COMP_WORDS[1]}"
    if [[ ${COMP_CWORD} -eq 1 ]]; then
        if [[ "$cur" == -* ]]; then
            COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
        else
            COMPREPLY=( $(compgen -W "assemble rule validate validate-ruling" -- "$cur") )
        fi
        return 0
    fi
    if [[ "$prev" == "--ruling" && "$cmd" == "rule" ]]; then
        COMPREPLY=( $(compgen -W "keep demote quarantine kill" -- "$cur") )
        return 0
    fi
    if [[ "$prev" == "--expires" || "$prev" == "--today" ]]; then
        return 0
    fi
    if [[ "$cur" == -* ]]; then
        case "$cmd" in
            assemble)
                COMPREPLY=( $(compgen -W "--catchledger --canaryfield --blindgate --decision --packs-dir -h --help" -- "$cur") )
                ;;
            rule)
                COMPREPLY=( $(compgen -W "--pack --ruling --expires --today --out -h --help" -- "$cur") )
                ;;
            validate)
                COMPREPLY=( $(compgen -W "--pack --catchledger --canaryfield --blindgate --decision -h --help" -- "$cur") )
                ;;
            validate-ruling)
                COMPREPLY=( $(compgen -W "--pack --ruling --body -h --help" -- "$cur") )
                ;;
            *)
                COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
                ;;
        esac
        return 0
    fi
    local IFS=$'\n'
    COMPREPLY=( $(compgen -f -- "$cur") )
}
complete -o filenames -F _tameshi_completions tameshi
