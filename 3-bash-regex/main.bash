#!/usr/bin/env bash

# Part 1

regex="mul\([0-9]{1,3},[0-9]{1,3}\)"

grep -E -o $regex input.txt \
    | tr --delete "mul" \
    | tr "," "*" \
    | tr "\n" "+" \
    | head -c -1 \
    | bc

# Part 2

regex="(mul\([0-9]{1,3},[0-9]{1,3}\))|do\(\)|don't\(\)"

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

