import sys

def check_number_of_v(nums, v):
    return nums.count(v)


if __name__ == '__main__':
    inputs = sys.stdin.read().splitlines()
    nums = list(map(int, inputs[1].split()))
    v = int(inputs[2])
    print(check_number_of_v(nums, v))