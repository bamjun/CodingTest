def hanoi(n, start, end, mid):
    if n == 1:
        print(start, end)
        return
    hanoi(n - 1, start, mid, end)
    print(start, end)
    hanoi(n - 1, mid, end, start)

# 입력
n = int(input())

# 총 이동 횟수 출력
print(2**n - 1)

# 하노이 탑 수행 과정 출력
hanoi(n, 1, 3, 2)
