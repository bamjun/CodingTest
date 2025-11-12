def solution(s, p):
    # pi 배열 계산
    pi = [0] * len(p)
    j = 0
    for i in range(1, len(p)):
        while j > 0 and p[i] != p[j]:
            j = pi[j - 1]
        if p[i] == p[j]:
            j += 1
            pi[i] = j

    # 검색
    j = 0
    for ch in s:
        while j > 0 and ch != p[j]:
            j = pi[j - 1]
        if ch == p[j]:
            j += 1
            if j == len(p):
                return 1  # 부분 문자열 발견
    return 0  # 못 찾음


if __name__ == '__main__':
    s = input().strip()
    p = input().strip()
    print(solution(s, p))
