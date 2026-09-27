import math
import sys

def getData(index, promt):
    try:
        coef = sys.argv[index]
    except:
        print(promt)
        coef = input()

    while True:
        try:
            coef = float(coef)
            return coef
        except:
            print(f"[ ERROR ] {promt}")
            coef = input()

def calcRoots(a, b, c):
    d = b**2 - 4 * a * c

    roots = []

    match d:
        case _ if d < 0:
            print("Корней нет, D < 0")
            return []
        case 0:
            rt = -b / (2 * a)
            if rt > 0:
                roots.append(math.sqrt(rt))
                roots.append(-math.sqrt(rt))
            elif rt == 0:
                roots.append(math.sqrt(rt))
        case _:
            d = math.sqrt(d)

            rt1 = (-b + d) / (2 * a)
            rt2 = (-b - d) / (2 * a)

            if (rt1 > 0):
                roots.append(math.sqrt(rt1))
                roots.append(-math.sqrt(rt1))
            elif (rt1 == 0):
                roots.append(0)

            if (rt2 > 0):
                roots.append(math.sqrt(rt2))
                roots.append(-math.sqrt(rt2))
            elif (rt2 == 0 and rt1 != 0):
                roots.append(0)

    return sorted(roots)


def test1():
    result = calcRoots(1, -5, 4)
    expected = [-2, -1, 1, 2]
    assert result == expected, f"Тест 1 провален: ждали {expected}, получили {result}"
    print("Тест на 4 корня пройден")

def test2():
    result = calcRoots(1, -4, 0)
    expected = [-2, 0, 2]
    assert result == expected, f"Тест 2 провален: ждали {expected}, получили {result}"
    print("Тест на 3 корня пройден")

def test3():
    result = calcRoots(1, -3, -4)
    expected = [-2, 2]
    assert result == expected, f"Тест 3 провален: ждали {expected}, получили {result}"
    print("Тест на 2 корня пройден")

def test4():
    result = calcRoots(1, 4, 0)
    expected = [0]
    assert result == expected, f"Тест 4 провален: ждали {expected}, получили {result}"
    print("Тест на 1 корень пройден")

def test5():
    result = calcRoots(1, 5, 4)
    expected = []
    assert result == expected, f"Тест 5 провален: ждали {expected}, получили {result}"
    print("Тест на отсутствие корней пройден")

def launchTests():
    test1()
    test2()
    test3()
    test4()
    test5()

if(__name__ == "__main__"):
    launchTests()

    a = getData(1, "Введите A:")
    b = getData(2, "Введите B:")
    c = getData(3, "Введите C:")
    print(calcRoots(a,b,c))
