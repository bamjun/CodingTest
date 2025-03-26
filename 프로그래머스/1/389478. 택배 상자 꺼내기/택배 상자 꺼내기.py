def solution(n, w, num):
    columns = [[] for _ in range(w)]
    current_box = 1
    layer = 0

    while current_box <= n:
        if layer % 2 == 0:  # 왼->오른
            for c in range(w):
                if current_box > n:
                    break
                columns[c].append(current_box)
                current_box += 1
        else:               # 오른->왼
            for c in range(w-1, -1, -1):
                if current_box > n:
                    break
                columns[c].append(current_box)
                current_box += 1
        layer += 1

    for c in range(w):
        if num in columns[c]:
            idx = columns[c].index(num)
            return len(columns[c]) - idx

    return 0  # 기본적으로 도달하지 않음
