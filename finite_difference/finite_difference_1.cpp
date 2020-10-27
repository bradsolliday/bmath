#include <Eigen/Sparse>
#include <Eigen/Dense>
#include <matplotlib-cpp/matplotlibcpp.h>
#include <vector>
#include <iostream>
#include <cmath>

namespace plt = matplotlibcpp;

double u(double x) {
    return std::cos(M_PI_2 * x);
}

double f(double x) {
    return M_PI_2 * M_PI_2 * std::cos(M_PI_2 * x);
}

int main() {
    using namespace Eigen;
    const int n = 100; // The number of points we evaluate at
    const double start = 0;
    const double end = 1;

    // Builging the top matrix T
    SparseMatrix<double> T(n, n);
    T.reserve(VectorXi::Constant(n, 3));
    T.insert(0, 0) =  1;
    T.insert(1, 0) = -1;
    for (int col = 1; col < n - 1; ++col) {
        T.insert(col - 1, col) = -1;
        T.insert(col    , col) =  2;
        T.insert(col + 1, col) = -1;
    }
    T.insert(n - 2, n - 1) = -1;
    T.insert(n - 1, n - 1) =  2;

    // Initializing the force vector F
    const double h = (end - start) / (n - 1.0);
    VectorXd F = VectorXd::Zero(n);
    double x = start;
    for (int i = 0; i < n; ++i) {
        F[i]= f(x);
        x += h;
    }

    // Solving
    SimplicialLLT<SparseMatrix<double>> solver;
    solver.compute(T);
    if (solver.info() != Success) {
        std::cout << "The solver failed at computing T\n";
        return 1;
    }
    VectorXd U;
    U = h * h * solver.solve(F);
    if (solver.info() != Success) {
        std::cout << "The solver failed at computing U\n";
        return 1;
    }

    std::vector<double> xpoints(n);
    std::vector<double> exact_soln(n);
    x = start;
    for (int i = 0; i < n; ++i) {
        xpoints[i] = x;
        exact_soln[i] = u(x);
        x += h;
    }

    std::vector<double> Upoints(U.data(), U.data() + U.size());

    plt::plot(xpoints, Upoints, "b");
    plt::plot(xpoints, exact_soln, "r+");
    plt::show();
}
