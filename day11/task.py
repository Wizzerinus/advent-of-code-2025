from collections.abc import Collection
import functools


def main():
    with open("input11.txt") as f:
        content = dict(x.split(": ") for x in f.readlines())
    
    @functools.cache
    def paths_from(current: str, remaining_visits: Collection[str]) -> int:
        if current == "out":
            return 1 if not remaining_visits else 0
        if current in remaining_visits:
            remaining_visits = tuple([x for x in remaining_visits if x != current])
        return sum(paths_from(x, remaining_visits) for x in content[current].split())
   
    day1 = paths_from("you", ())
    day2 = paths_from("svr", ("dac", "fft"))
    print(f"Day 1: {day1}")
    print(f"Day 2: {day2}")


if __name__ == "__main__":
    main()

