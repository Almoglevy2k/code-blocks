#!/bin/sh

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <examples-dir>"
    exit 1
fi

examples=$1

for example in $(ls $examples)
do
    # remove suffix
    name="${example%.*}"

    # replace _ with space
    name="${name//_/ }"

    echo "### $name"
    echo
    echo "![$name]($examples/$example)"
    echo

done