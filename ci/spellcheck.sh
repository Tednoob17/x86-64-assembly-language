#!/bin/bash

set -eu

aspell --version

shopt -s nullglob

dict_filename=./ci/dictionary.txt
markdown_sources=(./src/*.md)
mode="check"

dict_path="/tmp/dictionary.txt"

if [[ "${1:-}" == "list" ]]; then
    mode="list"
fi

if [[ "$mode" == "list" && ! -f "$dict_filename" ]]; then
    echo "No dictionary file found! A dictionary file is required in CI!"
    exit 1
fi

if [[ ! -f "$dict_filename" ]]; then
    echo "Scanning files to generate dictionary file '$dict_filename'."
    echo "Please check that it doesn't contain any misspellings."

    echo "personal_ws-1.1 en 0 utf-8" > "$dict_filename"
    cat "${markdown_sources[@]}" | aspell --ignore 3 list | sort -u >> "$dict_filename"
elif [[ "$mode" == "list" ]]; then
    declare -i retval=0

    cp "$dict_filename" "$dict_path"

    if [[ ! -f "$dict_path" ]]; then
        retval=1
        exit "$retval"
    fi

    for fname in "${markdown_sources[@]}"; do
        command=$(aspell --ignore 3 --personal="$dict_path" "$mode" < "$fname")
        if [[ -n "$command" ]]; then
            for error in $command; do
                grep --with-filename --line-number --color=always "$error" "$fname"
            done
            retval=1
        fi
    done
    exit "$retval"
elif [[ "$mode" == "check" ]]; then
    cp "$dict_filename" "$dict_path"

    if [[ ! -f "$dict_path" ]]; then
        retval=1
        exit "$retval"
    fi

    for fname in "${markdown_sources[@]}"; do
        aspell --ignore 3 --dont-backup --personal="$dict_path" "$mode" "$fname"
    done
fi