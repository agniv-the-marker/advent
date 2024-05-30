max_red = 12
max_green = 13
max_blue = 14

def valid_game(description, power = False):
    """
    returns ID if the game is valid, otherwise returns 0
    """
    description = description.strip()
    ID = int(description[4:description.index(':')])
    game = [subgame.split(',') for subgame in description[description.index(':')+2:].split(';')]
    max_red = 0
    max_green = 0
    max_blue = 0
    for subgame in game:
        for color in subgame:
            if color[-1] == 'd': # red case
                max_red = max(max_red, int(color[:-4]))
                if not power and int(color[:-4]) > max_red:
                    return 0
            elif color[-1] == 'n': # green case
                max_green = max(max_green, int(color[:-5]))
                if not power and int(color[:-5]) > max_green:
                    return 0
            elif color[-1] == 'e': # blue case
                max_blue = max(max_blue, int(color[:-4]))
                if not power and int(color[:-4]) > max_blue:
                    return 0
    if power:
        return max_red * max_green * max_blue
    return ID

valid_sum = 0
power_sum = 0

with open('../input/conundrum.txt') as f:
    for line in f:
        valid_sum += valid_game(line, False)
        power_sum += valid_game(line, True)

print(f'task 1: {valid_sum}\ntask 2: {power_sum}')