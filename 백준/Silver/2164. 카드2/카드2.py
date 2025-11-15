import sys
from collections import deque

def solution():
    input = sys.stdin.readline
    n = int(input().strip())
    
    cards = deque(range(1, n + 1))

    while len(cards) > 1:
        cards.popleft()
        cards.append(cards.popleft())

    return cards[0]

if __name__ == '__main__':
    answer = solution()
    print(answer)
