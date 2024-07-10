def is_prime_number(x):
    # 2부터 (x - 1)까지의 모든 수를 확인하며
    for i in range(2, x):
        # x가 해당 수로 나누어떨어진다면
        if x % i == 0:
            return False # 소수가 아님
    return True # 소수임

def solution(nums):
    test_index = 0
    answer = 0
    for a in range(len(nums)):
        for b in range(len(nums)):
            if a >= b:
                continue
            for c in range(len(nums)):
                if b >= c:
                    continue
                test_index = nums[a] + nums[b] + nums[c]
                
                if is_prime_number(test_index) == True:
                    answer += 1

    return answer
