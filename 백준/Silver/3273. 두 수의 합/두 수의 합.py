n = int(input())
l = list(map(int, input().split()))
x = int(input())
count = 0
seen = set()
for i in l:
    v = x - i
    if v in seen:
        count += 1
    seen.add(i)
print(count)
