def solution(n, w, num): 
    return sum((
        ((b - 1) % w if ((b - 1) // w) % 2 == 0 else w - 1 - ((b - 1) % w))
        ==
        ((num - 1) % w if ((num - 1) // w) % 2 == 0 else w - 1 - ((num - 1) % w))
    ) for b in range(num + 1, n + 1)) + 1

