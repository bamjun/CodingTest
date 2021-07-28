def solution(d, budget):
    d.sort()
    index = 0
    answer = 0
    xanswer = 0
    for x in d:
        xanswer = answer + x
        if xanswer > budget:
            return index
        else:
            answer = answer + x
            index = index + 1

    return index
