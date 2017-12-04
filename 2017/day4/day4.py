#!/usr/bin/env python


def is_anagram(a, b):
    ord_a = list(sorted(a))
    ord_b = list(sorted(b))

    return ord_a == ord_b

def analyse_part1(text):

    valid_lines = 0
    for line in text:
        if len(line) == len(set(line)):
            valid_lines += 1
    return valid_lines

def analyse_part2(text):

    valid_lines = 0

    for line in text:
        vals = set()
        found_anagram = False
        for word in line:
            for other in vals:
                if is_anagram(word, other):
                    found_anagram = True
                    break

            if found_anagram:
                break

            vals.add(word)

        if not found_anagram:
            valid_lines += 1

    return valid_lines


with open('input') as infile:
    lines = (line.strip().split() for line in infile)
    print('part1:',analyse_part1(lines))


with open('input') as infile:
    lines = (line.strip().split() for line in infile)
    print('part2:',analyse_part2(lines))
