import sys

def countPairsWithSum(nums, x):
    seen = set()
    count = 0
    
    for i in nums:
        target = x - i
        if target in seen:
            count += 1
        seen.add(i)
        
    return count

if __name__ == '__main__':
    # 모든 입력 읽기
    inputs = sys.stdin.read().splitlines()
    
    # 데이터 파싱
    n = int(inputs[0])  # 첫 번째 줄: 수열의 크기
    nums = list(map(int, inputs[1].split()))  # 두 번째 줄: 수열
    x = int(inputs[2])  # 세 번째 줄: 찾고자 하는 합

    # 함수 호출 및 결과 출력
    count = countPairsWithSum(nums, x)
    print(count)
