def get_first_and_last_digit(sequence, alpha = True):

    # alpha for first / second task

    sequence += " " * 4 # pad out

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
            for key in digit_mapping:
                if char == key[0] and sequence[i:i+len(key)] == key:
                    if first_digit is None:
                        first_digit = digit_mapping[key]
                    last_digit = digit_mapping[key]
                    break
    return int(first_digit + last_digit)

calibrations = 0
alpha_calibs = 0

with open("../input/trebuchet.txt") as f:
    for line in f:
        calibrations += get_first_and_last_digit(line.strip(), False)
        alpha_calibs += get_first_and_last_digit(line.strip(), True)

print(f'task 1: {calibrations}\ntask 2: {alpha_calibs}')