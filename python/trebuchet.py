"""
https://adventofcode.com/2023/day/1
"""

def get_first_and_last_digit(sequence, alpha=False):
    """
    Returns the first and last digit of a given sequence.

    Args:
        sequence (str): The input sequence.
        alpha (bool, optional): Specifies whether to consider alphabetic characters as digits. 
                                Defaults to False.

    Returns:
        int: The first and last digit of the sequence.

    Example:
        >>> get_first_and_last_digit("abc123def")
        13
        >>> get_first_and_last_digit("two123def", alpha=True)
        23
    """
    sequence += " " * 4  # pad out

    digit_mapping = {
        "zero": "0",
        "one": "1",
        "two": "2",
        "three": "3",
        "four": "4",
        "five": "5",
        "six": "6",
        "seven": "7",
        "eight": "8",
        "nine": "9"
    }

    first_digit = None
    last_digit = None
    for i, char in enumerate(sequence):
        if char.isdigit():
            if first_digit is None:
                first_digit = char
            last_digit = char
        elif char.isalpha() and alpha:
            for key, value in digit_mapping.items():
                if char == key[0] and sequence[i:i + len(key)] == key:
                    if first_digit is None:
                        first_digit = value
                    last_digit = value

    if not first_digit:
        first_digit = ''
    if not last_digit:
        last_digit = ''

    return int(first_digit + last_digit)

if __name__ == "__main__":
    CALIB = 0
    ALPHA = 0

    with open("../input/trebuchet.txt", encoding='utf-8') as f:
        for line in f:
            CALIB += get_first_and_last_digit(line.strip(), False)
            ALPHA += get_first_and_last_digit(line.strip(), True)

    print(f'task 1: {CALIB}\ntask 2: {ALPHA}')
