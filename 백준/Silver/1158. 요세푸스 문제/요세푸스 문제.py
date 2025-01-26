from collections import deque

def solve_case(N, K):
    list_of_case = deque(range(1, N + 1))
    result = []
    
    while list_of_case:
        list_of_case.rotate(-(K-1))
        result.append(list_of_case.popleft())
        
    return f"<{', '.join(map(str, result))}>"        
            
            
    

if __name__ == '__main__':
    N, K = map(int, input().strip().split())
    print(solve_case(N, K))