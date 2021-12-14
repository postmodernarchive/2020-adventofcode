from typing import List
from pathlib import Path


def get_lines(file: Path) -> List[str]:
    lines: List[str] = []

    with open(file, 'r', encoding='UTF-8') as f:
        for l in f.readlines():
            lines.append(l)

    return lines
