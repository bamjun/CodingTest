def solution(record):
    answer = []
    nick_change = {}

    for x in record:
        if x.split(' ')[0] == 'Enter':
            answer.append([x.split(' ')[1], "님이 들어왔습니다."])
            nick_change[x.split(' ')[1]] = x.split(' ')[2]
        elif x.split(' ')[0] == 'Leave':
            answer.append([x.split(' ')[1], "님이 나갔습니다."])
        else:
            nick_change[x.split(' ')[1]] = x.split(' ')[2]

    for y in range(len(answer)):
        answer[y][0] = nick_change[answer[y][0]]



    return [''.join(x) for x in answer]
