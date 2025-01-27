def solve_problem(list_of_case):
    results = []
    stacks = []
    for x in list_of_case:
        if 'push' in x:
            _, index = x.split()
            stacks.append(index)
        elif 'pop' in x:
            if stacks:
                results.append(stacks.pop())
            else:
                results.append('-1')
        elif 'size' in x:
            results.append(len(stacks))
        elif 'empty' in x:
            if stacks:
                results.append('0')
            else:
                results.append('1')
        elif 'top' in x:
            if stacks:
                results.append(stacks[-1])
            else:
                results.append('-1')
    return results
                
            
    


if __name__ == '__main__':
    number_of_case = int(input())
    list_of_case = [input().strip() for _ in range(number_of_case)]
    results = solve_problem(list_of_case)
    
    for result in results:
        print(result)