from typing import List

def list_of_strfry(list_of_test_case: List[List[str]]) -> List[str]:
    list_of_result = []
    
    for x, y in list_of_test_case:
        if sorted(x) == sorted(y):
            list_of_result.append("Possible")
        else:
            list_of_result.append("Impossible")
    
    return list_of_result

def data_parse_to_list(number_of_test_case: int) -> List[List[str]]:
    lists = []
    for _ in range(number_of_test_case):
        x, y = input().split()
        lists.append([x, y])
    return lists

if __name__ == '__main__':
    number_of_test_case = int(input())
    list_of_test_case = data_parse_to_list(number_of_test_case)
    list_of_result = list_of_strfry(list_of_test_case)
    
    for x in list_of_result:
        print(x)
