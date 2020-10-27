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

// Returns a sparse Top matrix (second difference matrix with
// u'' = 0 boundary condition on the top and u = 0 boundary condition
// on the bottom, where u is the function we are differentiating. The
// matrix is off the form)
// [  1 -1  0  0  0 ]
// [ -1  2 -1  0  0 ]
// [  0 -1  2 -1  0 ]
// [  0  0 -1  2 -1 ]
// [  0  0  0 -1  2 ]
// 
// Precondition: dimension >= 2
Eigen::SparseMatrix<double>
top_matrix(int dimension) {
    Eigen::SparseMatrix<double> out(dimension, dimension);
    out.reserve(Eigen::VectorXi::Constant(dimension, 3));
    out.insert(0, 0) = 1;
    out.insert(1, 0) = -1;
    for (int col = 1; col < dimension - 1; ++col) {
        out.insert(col - 1, col) = -1;
        out.insert(col    , col) =  2;
        out.insert(col + 1, col) = -1;
    }
    out.insert(dimension - 2, dimension - 1) = -1;
    out.insert(dimension - 1, dimension - 1) =  2;
    return out;
}

// Returns the vectorized form of the function f on the discretized interval
// starting at start with size steps of step_size length.
// f is meant to represent the force being applied in the differential equation
// we're solving.
// If you didn't define this function, the function f is almost certainly not
// the one you want and you'd be better off writing your own version.
//
// Precondition: size > 0
Eigen::VectorXd
force_vector(double start, double step_size, int size) {
    Eigen::VectorXd out = Eigen::VectorXd::Zero(size);
    double x = start;
    for (int i = 0; i < size; ++i) {
        out[i] = f(x);
        x += step_size;
    }
    return out;
}

// Plots the values of predicted with the spacing starting at start with step
// size step_size
void
plot_comparison_graph(
        const Eigen::VectorXd& predicted,
        double start,
        double step_size) {
    typedef std::vector<double> vecd;

    // Constructing the values of the exact solution u we wish to plot against
    const int e_eval_pts = 1000; //read e as exact
    const double e_start = 0;
    const double e_end = 1;
    vecd exact_x(e_eval_pts);
    vecd exact_soln(e_eval_pts);
    const double e_step_size = (e_end - e_start) / (e_eval_pts - 1);
    double xe = e_start;
    for (int i = 0; i < e_eval_pts; ++i) {
        exact_x[i] = xe;
        exact_soln[i] = u(xe);
        xe += e_step_size;
    }

    plt::plot(exact_x, exact_soln, "k");
        
    // Converting predicted to a std::vector<double and constructing the
    // x-values we are plotting it against
    const int predicted_pts = predicted.size();
    vecd predicted_soln(predicted.data(), predicted.data() + predicted_pts);
    vecd predicted_x(predicted_pts);
    double xp = start;
    for (int i = 0; i < predicted_pts; ++i) {
        predicted_x[i] = xp;
        xp += step_size;
    }

    plt::plot(predicted_x, predicted_soln, "r+");

    plt::show();
}

int main() {
    using namespace Eigen;
    const int n = 10; // The number of points we evaluate at
    const double start = 0;
    const double end = 1;

    // Building the top matrix T
    const SparseMatrix<double> T = top_matrix(n);

    // Initializing the force vector F
    const double h = (end - start) / (n + 0.5);
    const VectorXd F = force_vector(start + h/2, h, n);

    // Solving
    SimplicialLLT<SparseMatrix<double>> solver;
    solver.compute(T);
    if (solver.info() != Success) {
        std::cout << "The solver failed at computing T\n";
        return 1;
    }
    const VectorXd U = h * h * solver.solve(F);
    if (solver.info() != Success) {
        std::cout << "The solver failed at computing U\n";
        return 1;
    }

    plot_comparison_graph(U, start + h/2, h);
}
