# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Vladimir Rusinov
"""Text align justify.

https://www.codewars.com/kata/537e18b6147aa838f600001b/train/python
"""


def justify(text, width):
    words = text.split()
    ret = ""
    while len(words) > 0:
        line = []
        while words and len(" ".join(line + [words[0]])) <= width:
            line.append(words.pop(0))
        if len(words) == 0:
            # Last line, left justified
            ret += " ".join(line) + "\n"
        else:
            # Need to justify the line
            total_spaces = width - sum(len(word) for word in line)
            gaps = len(line) - 1
            if gaps > 0:
                spaces_per_gap = total_spaces // gaps
                extra_spaces = total_spaces % gaps
                justified_line = []
                for i in range(len(line) - 1):
                    justified_line.append(line[i])
                    justified_line.append(
                        " " * (spaces_per_gap + (1 if i < extra_spaces else 0))
                    )
                justified_line.append(line[-1])
                ret += "".join(justified_line) + "\n"
            else:
                ret += line[0] + "\n"
    if ret.endswith("\n"):
        ret = ret[:-1]
    return ret


if __name__ == "__main__":
    justify("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", 20)
    print(justify("123 45 6", 7))
