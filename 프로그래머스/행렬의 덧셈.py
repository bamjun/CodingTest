def solution(a, b):
    answer = []
    answer_index = []
    for x, y in zip(a,b):
        for xx, yy in zip(x, y):
            answer_index.append(xx+yy)
        answer.append(answer_index)
        answer_index = []
             
    return answer
