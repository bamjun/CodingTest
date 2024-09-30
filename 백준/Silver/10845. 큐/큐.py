n = int(input())

q = []
for _ in range(n):
    command = input().split()
    exe = command[0]
    
    if exe == "push":
        num = int(command[1])
        q.append(num)
    
    elif exe == "pop":
        if len(q) == 0:
            print(-1)
        else:
            print(q.pop(0))
    
    elif exe == "size":
        print(len(q))
    
    elif exe == "empty":
        print(1 if len(q) == 0 else 0)
    
    elif exe == "front":
        if len(q) == 0:
            print(-1)
        else:
            print(q[0])
    
    elif exe == "back":
        if len(q) == 0:
            print(-1)
        else:
            print(q[-1])
