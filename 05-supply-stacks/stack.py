#!/usr/bin/env python3

import sys

from collections import defaultdict


def main():
    stacks, cmds = open(sys.argv[1]).read().split("\n\n")
    cmds = cmds.strip().split("\n")

    D = defaultdict(list)
    for line in stacks.split("\n"):
        for col, letter in enumerate(line):
            if letter.isalpha() or letter.isnumeric():
                D[col].append(letter)

    for v in D.values():
        stack = v.pop()
        print(stack, "".join(v))

    print()
    for cmd in cmds:
        print(cmd)

if __name__ == "__main__":
    main()
