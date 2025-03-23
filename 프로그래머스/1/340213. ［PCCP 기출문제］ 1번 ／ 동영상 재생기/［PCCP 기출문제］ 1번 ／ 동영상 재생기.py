def to_seconds(t):
    m, s = map(int, t.split(":"))
    return m * 60 + s

def to_mmss(s):
    return f"{s // 60:02d}:{s % 60:02d}"

def solution(video_len, pos, op_start, op_end, commands):
    video_len = to_seconds(video_len)
    pos = to_seconds(pos)
    op_start = to_seconds(op_start)
    op_end = to_seconds(op_end)

    # 처음 위치가 오프닝이면 건너뜀
    if op_start <= pos <= op_end:
        pos = op_end

    for cmd in commands:
        if cmd == "prev":
            pos = max(pos - 10, 0)
        elif cmd == "next":
            pos = min(pos + 10, video_len)
        
        # 오프닝 구간이라면 오프닝 끝으로 건너뜀
        if op_start <= pos <= op_end:
            pos = op_end
    
    return to_mmss(pos)
