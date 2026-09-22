import math

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

    my_eq = BiquadraticEquation(1, -5, 4)

    print(solver.calcRoots(my_eq))
