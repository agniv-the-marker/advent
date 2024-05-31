"""
https://adventofcode.com/2023/day/2
"""

MAX_RED = 12
MAX_GREEN = 13
MAX_BLUE = 14

def valid_game(description, power = False):
    """
    returns ID if the game is valid, otherwise returns 0
    """
    description = description.strip()
    game = [subgame.split(',') for subgame in description[description.index(':')+2:].split(';')]
    cur_red = 0
    cur_green = 0
    cur_blue = 0
    for subgame in game:
        for color in subgame:
            if color[-1] == 'd': # red case
                cur_red = max(cur_red, int(color[:-4]))
                if not power and int(color[:-4]) > cur_red:
                    return 0
            elif color[-1] == 'n': # green case
                cur_green = max(cur_green, int(color[:-5]))
                if not power and int(color[:-5]) > cur_green:
                    return 0
            elif color[-1] == 'e': # blue case
                cur_blue = max(cur_blue, int(color[:-4]))
                if not power and int(color[:-4]) > cur_blue:
                    return 0
    if power:
        return cur_red * cur_green * cur_blue
    return int(description[4:description.index(':')])

if __name__ == "__main__":
    VALID_SUM = 0
    POWER_SUM = 0

    with open('../input/conundrum.txt', encoding='utf-8') as f:
        for line in f:
            VALID_SUM += valid_game(line, False)
            POWER_SUM += valid_game(line, True)

    print(f'task 1: {VALID_SUM}\ntask 2: {POWER_SUM}')
