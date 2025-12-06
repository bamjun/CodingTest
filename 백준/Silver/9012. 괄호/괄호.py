import sys

input = sys.stdin.readline

T = int(input().strip())

for _ in range(T):
    s = input().strip()
    cnt = 0
    is_vps = True

    for ch in s:
        if ch == '(':
            cnt += 1
        else:  
            cnt -= 1
            if cnt < 0:
                is_vps = False
                break

    if cnt != 0:
        is_vps = False

    print("YES" if is_vps else "NO")
