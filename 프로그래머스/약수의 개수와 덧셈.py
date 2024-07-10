def doong(n):return len([x for x in range(1, n+1) if n % x == 0 ])

def solution(left, right):
    answer = 0
    
    for x in range(left, right + 1):
        if (doong(x)%2) == 0:
            answer = answer + x
        else:
            answer = answer - x
    return answer
