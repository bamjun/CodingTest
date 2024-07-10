def solution(participant, completion):
    bb = 0
    dica = {}
    for x in participant:
        dica[hash(x)] = x
        bb += hash(x)
    
    for y in completion:
        bb -= hash(y)
        
    answer = dica[bb]
    return answer
