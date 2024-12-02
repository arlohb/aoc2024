#!/usr/bin/env bash

is_safe_part1() {
    local len=$#
    local report=($*)

    local direction=""
    local safe=true

    # Loop through indices
    for i in ${!report[@]}; do
        # Skip last
        if [ $i -eq $(($len - 1)) ]; then
            continue
        fi

        local a=${report[$i]}
        local b=${report[$(($i + 1))]}

        local diff=$((b - a))
        # Literally remove the '-' from the string
        local abs_diff=${diff#-}

        # A difference of 0 is unsafe
        if [ $diff -eq 0 ]; then
            safe=false
            break
        fi

        # A difference of >3 is unsafe
        if [ $abs_diff -gt 3 ]; then
            safe=false
            break
        fi

        if [ $diff -gt 0 ]; then
            if [ "$direction" = "decreasing" ]; then
                safe=false
                break
            fi
            direction="increasing"
        else
            if [ "$direction" = "increasing" ]; then
                safe=false
                break
            fi
            direction="decreasing"
        fi
    done

    echo $safe
}

main() {
    # Split into lines, outputting to $MAPFILE
    mapfile -t < input.txt
    lines=("${MAPFILE[@]}")

    local count=0

    for line in "${lines[@]}"; do
        local report=($line)

        # Just for debugging
        echo $(is_safe_part1 ${report[@]})

        if [ $(is_safe_part1 ${report[@]}) = true ]; then
            count=$(($count + 1))
        fi
    done

    echo $count
}

main

