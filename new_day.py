#!/bin/env python3

from glob import glob
from os import system, mkdir
from shutil import copy
from pathlib import Path

try:
    new_day = "day{}".format(sorted(int(d.replace("day", "")) for d in glob("day*")).pop() + 1)
except:
    new_day = "day1"
    mkdir("puzzles")

system(f"cargo init {new_day}")
system(f"cd {new_day} && cargo add --path ../utils utils")

with open(Path("puzzles") / f"{new_day}.txt", "a"):
    pass

with open(Path("puzzles") / f"{new_day}_test1.txt", "a"):
    pass

with open(Path("puzzles") / f"{new_day}_test2.txt", "a"):
    pass

with open(Path(f"{new_day}") / f"puzzle.txt", "a"):
    pass

copy("template.rs", Path(f"{new_day}") / "src" / "main.rs")