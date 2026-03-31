#!/usr/bin/env bash
set -euo pipefail

HEADER='/*
 * gh-notify-daemon - A simple github notification daemon
 * Copyright (C) 2026 2kybe3 <kybe@kybe.xyz>
 */'

for file in "$@"; do
    if head -n 10 "$file" | grep -Fq "gh-notify-daemon - A simple github notification daemon"; then
        continue
    fi

    tmp=$(mktemp)
    printf '%s\n\n' "$HEADER" > "$tmp"
    cat "$file" >> "$tmp"
    mv "$tmp" "$file"
done

