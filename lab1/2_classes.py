import math
import sys

class getData:
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
                coef = input(f"[ ERROR ] {promt}: ")

class BiquadraticEquation:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

class EquasionSolver:
    def calcRoots(self, eq):
        a = eq.a
        b = eq.b
        c = eq.c

        d = b**2 - 4 * a * c

        if (d < 0):
            print("Корней нет, D < 0")
            return []
        else:
            d = math.sqrt(d)
            roots = []

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

if(__name__ == "__main__"):
    solver = EquasionSolver()                   # EquasionSolver solver;

    a = getData.getData(1, "Введите A: ")
    b = getData.getData(2, "Введите B: ")
    c = getData.getData(3, "Введите C: ")

    my_eq = BiquadraticEquation(a, b, c)

    print(solver.calcRoots(my_eq))
