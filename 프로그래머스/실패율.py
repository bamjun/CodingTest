def solution(N, stages):
    arrival_index = 0
    clear_index = 0
    result = {}
    answer = []

    num = len(stages)

    for x in range(1, N + 1):
        count = stages.count(x)
        if num == 0:
            result[x] = 0
        else:
            result[x] = count / num

        num -= count


    for x, y in sorted(result.items(), key=lambda x: x[1]):
        answer.append(x)


    return answer
