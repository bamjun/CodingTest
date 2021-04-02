# 문제 설명
# 정수 배열 numbers가 주어집니다. numbers에서 서로 다른 인덱스에 있는 두 개의 수를 뽑아 더해서 만들 수 있는 모든 수를 배열에 오름차순으로 담아 return 하도록 solution 함수를 완성해주세요.
# 
# 제한사항
# numbers의 길이는 2 이상 100 이하입니다.
# numbers의 모든 수는 0 이상 100 이하입니다.
# 입출력 예
# numbers	result
# [2,1,3,4,1]	[2,3,4,5,6,7]
# [5,0,2,7]	[2,5,7,9,12]
# 입출력 예 설명
# 입출력 예 #1
# 
# 2 = 1 + 1 입니다. (1이 numbers에 두 개 있습니다.)
# 3 = 2 + 1 입니다.
# 4 = 1 + 3 입니다.
# 5 = 1 + 4 = 2 + 3 입니다.
# 6 = 2 + 4 입니다.
# 7 = 3 + 4 입니다.
# 따라서 [2,3,4,5,6,7] 을 return 해야 합니다.
# 입출력 예 #2
# 
# 2 = 0 + 2 입니다.
# 5 = 5 + 0 입니다.
# 7 = 0 + 7 = 5 + 2 입니다.
# 9 = 2 + 7 입니다.
# 12 = 5 + 7 입니다.
# 따라서 [2,5,7,9,12] 를 return 해야 합니다.

def solution(a):
    d = []
    for x in a:
        wi = a.index(x)
        xi = a.pop(wi)
        for y in a:
            ao = xi + y
            if ao in d:
                continue
            else:
                d.append(ao)
        a.insert(wi, xi)
    d.sort()
    return d

///////////////////////////////////////////////////////////////////////////////////////////////////////////////
def solution(numbers):
    """
    a = set([1, 1, 3])
    a = {1, 3}
    a = {1, 1, 3}
    a = {1, 3}
    sorted(a, reverse=True) 내림순정렬
    sorted(a, reverse=False) 오름순정렬 기본값default

    """
    i = 0
    j = 0
    s  = []
    for i in range(len(numbers)):
        print('1a', i, j, s)
        for j in range(len(numbers)):
            print('2a', i, j, s)
            if i < j:
                print('3aif문 발동', i, j, s)
                s.append(numbers[i] + numbers[j])

            print('4a두번째루프반복', i, j, s)
        print('5a첫번째루프반복', i, j, s)
    print('6a', i, j, s)
    return sorted(set(s))
///////////////////////////////////////////////////////////////////////////////////////////////////////////////
def solution(numbers): return sorted({numbers[i] + numbers[j] for i in range(len(numbers)) for j in range(len(numbers)) if i < j})


