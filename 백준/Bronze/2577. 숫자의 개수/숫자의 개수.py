abc = [int(input()) for _ in range(3)]
multi = str(abc[0] * abc[1] * abc[2])
for x in range(10):
    print(multi.count(str(x)))
    
    
    
