#!/usr/bin/env bash

regex_num="[0-9]{1,3}"
regex_mul="mul\($regex_num,$regex_num\)"

# Part 1

grep -E -o $regex_mul input.txt \
    | tr --delete "mul" \
    | tr "," "*" \
    | tr "\n" "+" \
    | head -c -1 \
    | bc

# Part 2

regex_do="do\(\)"
regex_dont="don't\(\)"
regex="$regex_mul|$regex_do|$regex_dont"

start="("
end=")"
do=")+1*("
dont=")+0*("

middle=$(grep -E -o $regex input.txt \
    | tr --delete "mul" \
    | tr "," "*" \
    | tr "\n" "+" \
    | head -c -1 \
    | sed "s/+do()+/$do/g" \
    | sed "s/+don't()+/$dont/g")

echo $start$middle$end | bc

