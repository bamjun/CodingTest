def score_grade(i):
    if i >= 90:
        return 'A'
    if i >= 80:
        return 'B'
    if i >= 70:
        return 'C'
    if i >= 50:
        return 'D'
    return 'F'


def solution(scores):
    test_index = []
    answer = []
    len_index = len(scores[0])


    for index, a in enumerate(scores[0]):
        for b in scores:
            test_index.append(b[index])

        dis1 = test_index[index]
        test_index.sort()
        print(test_index)

        if test_index[0] == dis1:
            if test_index[1] == dis1:
                answer.append(score_grade(sum(test_index) / len_index))
                test_index = []
                continue
            else:
                test_index.pop(0)
                answer.append(score_grade(sum(test_index) / (len_index - 1)))
                test_index = []
                continue

        if test_index[len_index - 1] == dis1:
            if test_index[len_index - 2] == dis1:
                answer.append(score_grade(sum(test_index) / len_index))
                test_index = []
                continue
            else:
                test_index.pop()
                answer.append(score_grade(sum(test_index) / (len_index - 1)))
                test_index = []
                continue

        answer.append(score_grade(sum(test_index) / len_index))
        test_index = []



    return ''.join(answer)
