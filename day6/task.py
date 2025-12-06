from functools import reduce


def process(items: list[int], opnd: str):
    match opnd:
        case "+":
            return sum(items)
        case "*":
            return reduce(lambda p, q: p * q, items, 1)
        case _:
            raise ValueError(f"unknown operation: {opnd}")


def main():
    with open("./input6.txt") as f:
        contents = [x.strip("\n") for x in f.readlines()]

    split_contents = [x.split() for x in contents]
    op_row = split_contents.pop()
    day1, day2, pos = 0, 0, 0
    for i, x in enumerate(op_row):
        human_numbers = [int(row[i]) for row in split_contents]
        day1 += process(human_numbers, x)
        maxlen = max(len(row[i]) for row in split_contents)
        strings = [row[pos : pos + maxlen] for row in contents[:-1]]
        cephalid_numbers = [int("".join(j[i] for j in strings)) for i in range(maxlen)]
        day2 += process(cephalid_numbers, x)
        pos += maxlen + 1

    print(f"Day 1: {day1}")
    print(f"Day 2: {day2}")


if __name__ == "__main__":
    main()
