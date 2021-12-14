from typing import List

from utils import get_lines


def count_increases(arr: List[int]) -> int:
    increases = 0
    previous_value = arr[0]
    arr.pop(0)
    for value in arr:
        if value > previous_value:
            increases += 1

        previous_value = value

    return increases


if __name__ == '__main__':
    depths = []

    for l in get_lines('input/01.txt'):
        depths.append(int(l))

    # part one
    increases = count_increases(depths)
    print(increases)

    # part two
    meas_windows = []
    for i in range(len(depths) - 2):
        meas_windows.append((depths[i], depths[i+1], depths[i+2]))

    summed_windows = []
    for window in meas_windows:
        total = 0
        for d in window:
            total += d

        summed_windows.append(total)

    print(f'window increases: {count_increases(summed_windows)}')
