def compute_prefix_function(s):
    n = len(s)
    pi = [0] * n
    k = 0
    
    for i in range(1, n):
        while k > 0 and s[k] != s[i]:
            k = pi[k - 1]
        if s[k] == s[i]:
            k += 1
        pi[i] = k
    
    return pi

def find_repetitions(s):
    n = len(s)
    pi = compute_prefix_function(s)
    
    len_prefix_suffix = n - pi[-1]
    if n % len_prefix_suffix == 0:
        return n // len_prefix_suffix
    else:
        return 1

def main():
    import sys
    input = sys.stdin.read
    data = input().strip().split()
    
    for s in data:
        if s == '.':
            break
        print(find_repetitions(s))

if __name__ == "__main__":
    main()
