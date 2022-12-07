#!/usr/bin/env python3

import re

import itertools

def parse_cargo_bay(s):

    pattern_idx_line = re.compile(r"^(\s*\d+)+")

    line_iter = reversed(s.splitlines())

    while not pattern_idx_line.match(next(line_iter)):
        pass

    pattern_first_line = re.compile(r"(\s*)\[\w\]")

    first_line = next(line_iter)
    if not (matches := pattern_first_line.findall(first_line)):
        raise Exception("Invalid cargo bay")

    stack_count = len(matches)
    cargo_bay = [[] for _ in range(stack_count)]

    line_iter = itertools.chain([first_line], line_iter)

    pattern_str = ''.join(
        "(?:" + m + r"(?:   |\[(\w)\])" for m in matches
    ) + ")?" * len(matches)

    print('Pattern:', pattern_str)
    pattern = re.compile(pattern_str)

    for line in line_iter:
        if not (matches := pattern.match(line)):
            break

        matches = matches.groups()
        for match, stack in zip(matches, cargo_bay):
            if match:
                stack.append(match)

    return cargo_bay


def main():

    s1 = """
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
    """
    cb1 = parse_cargo_bay(s1)
    assert(cb1 == [
        ['Z', 'N'],
        ['M', 'C', 'D'],
        ['P'],
    ])

    s2 = """
            [D]
            [D]
        [N] [C] [F]
        [Z] [M] [P] [A] [B]
         1   2   3  10  412
    """
    cb2 = parse_cargo_bay(s2)
    assert(cb2 == [
        ['Z', 'N'],
        ['M', 'C', 'D', 'D'],
        ['P', 'F'],
        ['A'],
        ['B'],
    ])


if __name__ == '__main__':
    main()
