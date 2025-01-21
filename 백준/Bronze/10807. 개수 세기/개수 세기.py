import sys

found = 0
num = int(sys.stdin.readline().strip())  # 정수로 변환
numlist = sys.stdin.readline().split()  # 리스트로 변환
findNum = sys.stdin.readline().strip()  # 오른쪽 공백 제거

for i in range(num):  # num을 정수로 사용
    if numlist[i] == findNum:  # 문자열 비교
        found += 1

print(found)  # 정수는 str 변환 없이 출력 가능
