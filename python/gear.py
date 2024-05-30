def find_parts(board):
    # board = list of strings of the form ".... num .... (@#* parts)..."
    board = ["." + line.strip() + "." for line in board]
    board = ["." * len(board[0])] + board + ["." * len(board[0])]
    parts = []
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
                            parts.append((int(number), i, j))
                            break
                    if len(parts) and parts[-1] == (int(number), i, j):
                        break
            if length:
                j += length
                length = 0
            else:
                j += 1
    return parts

if __name__ == "__main__":
    part_sum = 0

    with open("../input/gear.txt") as f:
        parts = find_parts(f.readlines())
        part_sum = sum([part[0] for part in parts])

    print(f'task 1: {part_sum}')
