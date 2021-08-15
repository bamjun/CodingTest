def solution(a, b, c):return max(0,sum([a * x for x in range(1 , c + 1)]) - b)
