import itertools


def is_valid_square(y, x):
    return y >= 0 and y < 8 and x >= 0 and x < 8


def list_to_hex(li):
    return hex(sum(map(lambda t: t[1]*(2**t[0]), enumerate(li))))


def generate_patterns():
    patterns = list()

    for y in range(8):
        for x in range(8):
            pattern = [0] * 64

            for dy, dx in itertools.product([-2, 2], [-1, 1]):
                if is_valid_square(y + dy, x + dx):
                    idx = (y + dy) * 8 + (x + dx)
                    pattern[idx] = 1

            for dy, dx in itertools.product([-1, 1], [-2, 2]):
                if is_valid_square(y + dy, x + dx):
                    idx = (y + dy) * 8 + (x + dx)
                    pattern[idx] = 1

            patterns.append(pattern)

    return list(map(list_to_hex, patterns))

if __name__ == '__main__':
    print(', '.join(generate_patterns()))
