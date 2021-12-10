vertical = 0
horizontal = 0
aim = 0


def main():

    # part 1
    #def up(v: int):
    #    global vertical
    #    vertical -= v
    #def down(v: int):
    #    global vertical
    #    vertical += v
    #def forward(v: int):
    #    global horizontal
    #    horizontal += v
    #

    def up(v: int):
        global aim
        aim -= v
    def down(v: int):
        global aim
        aim += v
    def forward(v: int):
        global horizontal
        global vertical
        global aim
        horizontal += v
        vertical += (aim * v)

    operations = {
        'up': up,
        'down': down,
        'forward': forward,
    }

    with open('input.txt', 'r', encoding='UTF-8') as input:
        i = 0
        for l in input.readlines():
            operation, value = l.split(' ')
            operations[operation](int(value))
            print(f'o {operation} val {value}\n\tv {vertical}, h {horizontal}')

            if i >= 10:
                continue
            i += 1

    print(f'h {horizontal} * v {vertical} = {horizontal * vertical}')


if __name__ == '__main__':
    main()
