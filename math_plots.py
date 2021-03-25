import matplotlib.pyplot as plt
import numpy as np


def coefficients_from_points(pair_list):
    pair_list = np.array(pair_list)
    k = len(pair_list)
    power_matrix = np.zeros((k, k))
    for row in range(0, k):
        for col in range(0, k):
            power_matrix[row, col] = pair_list[row, 0]**col
    value_vector = pair_list[:, 1]
    coefficients = np.linalg.solve(power_matrix, value_vector)
    return coefficients

def polynomial_from_coefficients(coefficients):
    k = len(coefficients)
    return lambda x: np.dot(coefficients, [x**n for n in range(0, k)])

def between(x, interval):
    a, b = interval
    return min(a, b) <= x and x <= max(a, b)

# Returns x in [a, b] such that for some x' in [a, b] s.t
# f(x') = y, |x - x'| < errors[0] and |f(x) - y| < errors[1]
# Precondition: for (a, b) = interval
# min(f(a), f(b)) <= y <= max(f(a), f(b))
def bisection(f, y, interval, errors):
    assert(between(y, (f(interval[0]), f(interval[1]))))
    a, b = interval
    while (b - a) > errors[0] or abs(f(a) - y) > errors[1]:
        half = (a + b)/2
        if between(y, (f(a), f(half))):
            b = half
        else:
            a = half
    return a

if __name__ == '__main__':

    plt.subplot(1, 2, 1)
    t = np.linspace(0, 3*np.pi, 1000)
    f = lambda x: 1.1*x + np.sin(x)
    ft = f(t)
    plt.plot(t, f(t), label='f')
    t_inverse = np.linspace(min(ft), max(ft), 100)
    f_inverse = np.vectorize(lambda y: bisection(f, y, [0, 10], [.01, .01]), otypes=[float])
    plt.plot(t_inverse, f_inverse(t_inverse), label='$f^{-1}$')
    plt.plot(t, f_inverse(f(t)), label='$f^{-1}\circ f$')
    plt.title("Numerically Inverting Functions")
    plt.axis('square')
    plt.legend(loc='upper center', bbox_to_anchor=(.85, 0.5), shadow=True, ncol=1)

    plt.subplot(1, 2, 2)
    pair_list = np.array([(1, 7), (2, 5), (3, 6), (4, 4), (6, 4), (7, 7), (8, 4)])
    poly = np.vectorize(
            polynomial_from_coefficients(coefficients_from_points(pair_list)),
            otypes=[float]
            )
    plt.plot(t, poly(t), color='k')
    plt.plot(pair_list[:, 0], pair_list[:, 1], 'o', color='k')
    plt.title("Polynomials From Values")
    plt.ylim(0, max(t))
    plt.xlim(0, max(t))
    plt.gca().set_aspect('equal', adjustable='box')

    plt.show()

