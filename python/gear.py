"""
https://adventofcode.com/2023/day/3
"""

from collections import defaultdict as ddict

def find_parts(board, gears=True):
    """
    Finds and returns the parts and gear ratios in a given board.

    Args:
        board (list): A list of strings representing the board.
        gears (bool, optional): Specifies whether to include gear ratios. 
            Defaults to True.

    Returns:
        dict: A dictionary containing the parts and gear ratios.
            If gears is False, returns a dictionary of parts where the keys are 
            the coordinates (x, y) and the values are the part numbers.
            If gears is True, returns a dictionary of gear ratios where the keys 
            are the coordinates (x, y) and the values are lists of adjacent gear 
            numbers.

    Example:
        board = [
            ".....",
            ".123.",
            ".4*5.",
            ".678.",
            "....."
        ]
        parts = find_parts(board, gears=False)
        print(parts)
        # Output: {(1, 1): 123, (2, 1): 4, (2, 2): 5, (2, 3): 678}

        ratios = find_parts(board, gears=True)
        print(ratios)
        # Output: {}
    """
    board = ["." + line.strip() + "." for line in board]
    board = ["." * len(board[0])] + board + ["." * len(board[0])]
    parts = ddict(int) # (x, y) -> part number
    ratios = ddict(list) # (x, y) -> list of adjacent numbers
    for i, row in enumerate(board):
        j = 0
        length = 0
        while j < len(row):
            length = 0
            while row[j + length].isdigit():
                length += 1
            if length == 0:
                j += 1
                continue
            number = row[j:j + length]
            # check around the number
            for x in range(-1, length + 1):
                for y in range(-1, 2):
                    if board[i + y][j + x] != "." and not board[i + y][j + x].isdigit():
                        parts[(i - 1, j - 1)] = int(number)
                    if board[i + y][j + x] == "*":
                        ratios[(i + y - 1, j + x - 1)].append(int(number))
            j += length
            length = 0
    if not gears:
        return parts
    return {k: v for k, v in ratios.items() if len(v) == 2}

if __name__ == "__main__":
    PART_SUM = 0
    GEAR_SUM = 0

    with open("../input/gear.txt", encoding='utf-8') as f:
        data = f.readlines()
        PART_SUM = sum(find_parts(data, False).values())
        GEAR_SUM = sum(v[0] * v[1] for (_, v) in find_parts(data, True).items())

    print(f'task 1: {PART_SUM}')
    print(f'task 2: {GEAR_SUM}')
