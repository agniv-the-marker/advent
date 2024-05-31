"""
https://adventofcode.com/2023/day/5
"""

def short_order(data):
    """
    Calculate the minimum order value after applying a series of transformations.

    Args:
        data (list): A list of strings containing the input data.

    Returns:
        int: The minimum order value after applying the transformations.
    """
    orders = list(map(int, data[0][7:].split(" ")))
    maps = data[1:]
    for almanac in maps:
        ranges = [list(map(int, triple.split())) for triple in almanac.split("\n")[1:]]
        for ind, order in enumerate(orders):
            for (
                dest_start,
                source_start,
                range_len,
            ) in ranges:  # should use a smarter data structure
                if source_start <= order < source_start + range_len:
                    orders[ind] = dest_start + order - source_start
    return min(orders)

def range_order(data):
    """
    Calculates the minimum order assuming the fertilizer is applied in ranges.

    Args:
        data (list): A list of strings containing the input data.

    Returns:
        int: The minimum order value after applying the transformations.
    """
    orders = list(map(int, data[0][7:].split(" ")))
    return orders[0]

with open("../input/fertilizer.txt", encoding="utf=8") as f:
    file_data = f.read().split("\n\n")
    print(f"task 1: {short_order(file_data)}")
    print(f"task 2: {range_order(file_data)}")
