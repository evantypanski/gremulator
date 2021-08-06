#!/bin/bash

if [ $# -eq 0 ]
then
    echo "USAGE: Call with a file <foo>.asm to get <foo>.gb rom";
    exit 0;
fi

if [ $# -ne 1 ]
then
    echo "ERROR: Too many arguments";
    exit 1;
fi

filename="$1"

name=$(echo "$filename" | cut -f 1 -d '.')

rgbasm -o $name.o $filename || { exit 1; }
rgblink -o $name.gb $name.o || { exit 1; }
