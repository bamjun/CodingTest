def find_password(test_cases):
    results = []

    for keys in test_cases:
        left_stack = []  # 커서 왼쪽의 문자들
        right_stack = []  # 커서 오른쪽의 문자들

        for key in keys:
            if key == '-':
                # 백스페이스: 왼쪽 스택의 마지막 문자를 제거
                if left_stack:
                    left_stack.pop()
            elif key == '<':
                # 커서 왼쪽 이동: 왼쪽 스택에서 오른쪽 스택으로 이동
                if left_stack:
                    right_stack.append(left_stack.pop())
            elif key == '>':
                # 커서 오른쪽 이동: 오른쪽 스택에서 왼쪽 스택으로 이동
                if right_stack:
                    left_stack.append(right_stack.pop())
            else:
                # 일반 문자 입력: 왼쪽 스택에 추가
                left_stack.append(key)

        # 최종 결과: 왼쪽 스택 + 오른쪽 스택 (오른쪽 스택은 뒤집어서 합침)
        results.append(''.join(left_stack) + ''.join(reversed(right_stack)))

    return results

if __name__ == "__main__":
    # 입력 받기
    t = int(input())  # 테스트 케이스 개수
    test_cases = [input().strip() for _ in range(t)]

    # 비밀번호 계산
    results = find_password(test_cases)

    # 출력
    for result in results:
        print(result)
