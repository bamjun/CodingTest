import sys
from collections import deque

def main():
    input = sys.stdin.readline
    q = deque()
    n = int(input())
    out = []

    for _ in range(n):
        cmd = input().split()

        if cmd[0] == "push":
            q.append(cmd[1])

        elif cmd[0] == "pop":
            out.append(q.popleft() if q else "-1")

        elif cmd[0] == "size":
            out.append(str(len(q)))

        elif cmd[0] == "empty":
            out.append("0" if q else "1")

        elif cmd[0] == "front":
            out.append(q[0] if q else "-1")

        elif cmd[0] == "back":
            out.append(q[-1] if q else "-1")

    return "\n".join(out)


if __name__ == "__main__":
    print(main())
