"""
시저 암호
문제 설명
어떤 문장의 각 알파벳을 일정한 거리만큼 밀어서 다른 알파벳으로 바꾸는 암호화 방식을 시저 암호라고 합니다. 예를 들어 "AB"는 1만큼 밀면 "BC"가 되고, 3만큼 밀면 "DE"가 됩니다. "z"는 1만큼 밀면 "a"가 됩니다. 문자열 s와 거리 n을 입력받아 s를 n만큼 민 암호문을 만드는 함수, solution을 완성해 보세요.

제한 조건
공백은 아무리 밀어도 공백입니다.
s는 알파벳 소문자, 대문자, 공백으로만 이루어져 있습니다.
s의 길이는 8000이하입니다.
n은 1 이상, 25이하인 자연수입니다.
입출력 예
s	n	result
"AB"	1	"BC"
"z"	1	"a"
"a B z"	4	"e F d"
"""

import string
def solution(s, n):
    alpa = list(string.ascii_lowercase)
    Alpa = list(string.ascii_uppercase)
    ss = list(s)
    answer = []

    for x in ss:
        if x == ' ':
            answer.append(x)
        elif x in alpa:
            if (alpa.index(x) + n) > 25:
                answer.append(alpa[(alpa.index(x) + n) % 25 - 1])
            else:
                answer.append(alpa[alpa.index(x) + n])
        else:
            if (Alpa.index(x) + n) > 25:
                answer.append(Alpa[(Alpa.index(x) + n) % 25 - 1])
            else:
                answer.append(Alpa[Alpa.index(x) + n])
    return ''.join(answer)
