"""
https://adventofcode.com/2023/day/4
"""

from collections import Counter as counter

def process_card(card: str):
    """
    Process a scratch card and calculate the score.

    Args:
        card (str): The scratch card string in the format "Card ID: Winning Numbers | Numbers".

    Returns:
        int: The calculated score based on the winning and entered numbers.

    Example:
        # Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        >>> process_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        8
    """
    card = card[card.index(':')+2:]
    winning, numbers = card.split('|')
    winning = set(map(int, winning.split()))
    numbers = counter(map(int, numbers.split()))
    score = 0
    for num, times in numbers.items():
        if num in winning:
            score += times
    return int(2**(score - 1)), score

if __name__ == "__main__":
    TOTAL_POINTS = 0
    TOTAL_CARDS  = 0
    with open("../input/scratchcard.txt", encoding='utf-8') as f:
        multiples = counter() # Note only have to keep track of 5 cards at a time
        for ind, line in enumerate(f):
            if multiples[ind-1]:
                del multiples[ind-1] # remove the previous card
            multiples[ind] += 1
            TOTAL_CARDS += multiples[ind]
            row_points, row_score = process_card(line.strip())
            for next_card in range(row_score):
                # have to multiply by # of cards for task 2
                multiples[ind + 1 + next_card] += multiples[ind]
            TOTAL_POINTS += row_points # do not need to multiply by # of cards for task 1
    print(f'task 1: {TOTAL_POINTS}')
    print(f'task 2: {TOTAL_CARDS}')
    