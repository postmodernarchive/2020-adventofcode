from utils import get_lines


def main():
    lines = get_lines('input/03.txt')
    line_len = len(lines[0])
    print(line_len)

    # only counts 1 bits
    bit_counter = [0] * line_len

    line_c = 0
    for l in lines:
        for i in range(line_len):
            if l[i] == '1':
                bit_counter[i] += 1
        line_c += 1

    # most common,least common bit
    gamma_rate, epsilon_rate = 0, 0

    # most common bit calc
    for i in range(line_len):
        gamma_rate <<= 1
        epsilon_rate <<= 1

        if bit_counter[i] >= int(line_c / 2):
            gamma_rate += 1
        else:
            epsilon_rate += 1

    print(f'gamma rate {gamma_rate} * epsilon rate {epsilon_rate} = {gamma_rate * epsilon_rate}')


if __name__ == '__main__':
    main()
