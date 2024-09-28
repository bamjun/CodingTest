import sys
A,B,C=map(int, sys.stdin.readline().split());

def power_mod(A, B, C):
    # Base case
    if B == 0:
        return 1
    # 분할 정복을 이용한 거듭제곱
    half = power_mod(A, B // 2, C)
    half = (half * half) % C
    if B % 2 == 0:
        return half
    else:
        return (half * A) % C

# 결과 출력
print(power_mod(A, B, C))
