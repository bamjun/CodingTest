def solution(wallet, bill):
    answer = 0

    # wallet과 bill의 가로, 세로 길이를 정렬하여 작은 값이 앞에 오도록 합니다.
    wallet.sort()
    bill.sort()

    # 지폐의 작은 값이 지갑의 작은 값보다 크거나
    # 지폐의 큰 값이 지갑의 큰 값보다 큰 동안 반복합니다.
    while bill[0] > wallet[0] or bill[1] > wallet[1]:
        # 지폐의 큰 값을 반으로 접습니다.
        bill[1] //= 2
        # 접은 후 지폐의 가로, 세로 길이를 다시 정렬하여 작은 값이 앞에 오도록 합니다.
        bill.sort()
        # 접은 횟수를 증가시킵니다.
        answer += 1

    return answer
