#!/bin/bash

total_files_removed=0
total_bytes_removed=0

format_size() {
    awk -v bytes="$1" 'BEGIN {
        if (bytes >= 1073741824)
            printf "%.1fGiB", bytes / 1073741824
        else if (bytes >= 1048576)
            printf "%.1fMiB", bytes / 1048576
        else if (bytes >= 1024)
            printf "%.1fKiB", bytes / 1024
        else
            printf "%dB", bytes
    }'
}

for dir in */; do
    if [ -f "${dir}Cargo.toml" ]; then
        echo "Cleaning ${dir}"

        output=$(cd "$dir" && cargo clean 2>&1)
        echo "$output"

        files=$(echo "$output" | awk '/Removed/ {print $2}')
        size=$(echo "$output" | awk '/Removed/ {
            value=$4
            if (value ~ /GiB/) {sub(/GiB/, "", value); value*=1073741824}
            else if (value ~ /MiB/) {sub(/MiB/, "", value); value*=1048576}
            else if (value ~ /KiB/) {sub(/KiB/, "", value); value*=1024}
            else {sub(/B/, "", value)}
            print value
        }')

        files=${files:-0}
        size=${size:-0}

        total_files_removed=$((total_files_removed + files))
        total_bytes_removed=$(awk -v a="$total_bytes_removed" -v b="$size" \
            'BEGIN { print a + b }')

        echo
    fi
done

echo "All Rust projects cleaned"
echo "Total files removed: ${total_files_removed}"
echo "Total size removed: $(format_size "$total_bytes_removed")"