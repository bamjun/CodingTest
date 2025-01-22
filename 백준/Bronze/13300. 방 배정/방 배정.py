import math

# 입력 받기
N, K = map(int, input().split())
students = [[0] * 7 for _ in range(2)]  # [성별][학년] 배열 생성

# 학생 정보 입력 받기
for _ in range(N):
    S, Y = map(int, input().split())
    students[S][Y] += 1

# 필요한 방 계산
rooms = 0
for S in range(2):  # 성별 0, 1
    for Y in range(1, 7):  # 학년 1~6
        rooms += math.ceil(students[S][Y] / K)

# 결과 출력
print(rooms)
