from collections import defaultdict as ddict

def find_parts(board, gears = True):
    board = ["." + line.strip() + "." for line in board]
    board = ["." * len(board[0])] + board + ["." * len(board[0])]
    parts = ddict(int) # (x, y) -> part number
    ratios = ddict(list) # (x, y) -> list of adjacent numbers
    for i, row in enumerate(board):
        j = 0
        length = 0
        while j < len(row):
            if row[j].isdigit():
                length = 0
                while row[j + length].isdigit():
                    length += 1
                number = row[j:j + length]
                # check around the number
                for x in range(-1, length + 1):
                    for y in range(-1, 2):
                        if board[i + y][j + x] != "." and not board[i + y][j + x].isdigit():
                            parts[(i - 1, j - 1)] = int(number)
                        if board[i + y][j + x] == "*":
                            ratios[(i + y - 1, j + x - 1)].append(int(number))
            if length:
                j += length
                length = 0
            else:
                j += 1
    if not gears:
        return parts
    return {k: v for k, v in ratios.items() if len(v) == 2}

if __name__ == "__main__":
    part_sum = 0
    gear_sum = 0

    with open("../input/gear.txt") as f:
        board = f.readlines()
        parts  = find_parts(board, False)
        ratios = find_parts(board, True)
        part_sum = sum(parts.values())
        gear_sum = sum([v[0] * v[1] for _, v in ratios.items()])

    print(f'task 1: {part_sum}')
    print(f'task 2: {gear_sum}')
