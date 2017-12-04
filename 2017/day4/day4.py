#!/usr/bin/env python


def analyse_part1(text):

    valid_lines = 0
    for line in text:
        if len(line) == len(set(line)):
            valid_lines += 1
    return valid_lines


with open('input') as infile:
    lines = (line.strip().split() for line in infile)
    print(analyse_part1(lines))
