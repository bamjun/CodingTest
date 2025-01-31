def stack_sequence(n, sequence):
    stack = []
    result = []
    current = 1 

    for num in sequence:
        while current <= num: 
            stack.append(current)
            result.append("+")
            current += 1

        if stack and stack[-1] == num:  
            stack.pop()
            result.append("-")
        else:  
            return ["NO"]

    return result


if __name__ == "__main__":
    n = int(input())  
    sequence = [int(input()) for _ in range(n)]

    output = stack_sequence(n, sequence)
    for op in output:
        print(op)
