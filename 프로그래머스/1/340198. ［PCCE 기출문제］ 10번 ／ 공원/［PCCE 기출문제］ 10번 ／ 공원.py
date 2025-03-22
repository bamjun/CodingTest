def solution(mats, park):
    mats.sort(reverse=True)
    rows, cols = len(park), len(park[0])
    
    for mat_size in mats:
        for r in range(rows - mat_size + 1):
            for c in range(cols - mat_size + 1):
                if all(park[i][j] == "-1" for i in range(r, r + mat_size) for j in range(c, c + mat_size)):
                    return mat_size
    
    return -1