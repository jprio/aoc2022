import ast
import itertools
input = open("src/input.txt", "r").readlines()
tuples = []


def compare(l, r):
    print(type(l), type(r))
    print(f"comparaison de {l} et {r}")
    ret = True
    if r == None:
        return False
    if (type(l) == int) & (type(r) == int):
        print("INTTTTTTT")
        if (l > r):
            print("return false")
            return False
        elif (l < r):
            return True
        elif l == r:
            return None
    if (type(l) == list) & (type(r) == int):
        r = [r]
        compare(l, r)
    if (type(l) == int) & (type(r) == list):
        l = [l]
        compare(l, r)
    if (type(l) == list) & (type(r) == list):
        for a, b in zip(l, r):
            ret = compare(a, b)
            if ret == False:
                break

    print("return true")
    return ret


for i in range(0, int(len(input)/3), 3):
    print(30*"=")
    tuple = (
        ast.literal_eval(input[i].replace('\n', '')),
        ast.literal_eval(input[i+1].replace('\n', '')),
    )
    iter_result = itertools.zip_longest(ast.literal_eval(input[i].replace('\n', '')),
                                        ast.literal_eval(input[i+1].replace('\n', '')),)
    greater = True
    for l, r in iter_result:
        greater = compare(l, r)
        print("iter_result : " + str(greater))
        # if (not (greater == None)):
        #    break
        if (greater == False):
            break
    print(str(i) + ": " + str(greater))
    #print(type(l), type(r))
    # print(list(iter_result))
    # print(list(iter_result)[1])
    tuples.append(tuple)
    #print(str(tuple) + " " + str(greater))
