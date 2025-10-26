import sys, heapq

input = sys.stdin.readline

def main():
    N, M = map(int, input().split())
    graph = [[] for _ in range(N + 1)]
    indeg = [0] * (N + 1)
    
    for _ in range(M):
        a, b = map(int, input().split())
        graph[a].append(b)
        indeg[b] += 1
    
    heap = []
    for i in range(1, N + 1):
        if indeg[i] == 0:
            heapq.heappush(heap, i)
            
    order = []
    while heap:
        cur = heapq.heappop(heap)
        order.append(cur)
        for nxt in graph[cur]:
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                heapq.heappush(heap, nxt)
                
    print(*order)
    

if __name__ == "__main__":
    main()