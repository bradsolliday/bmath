import matplotlib.pyplot as plt
from matplotlib.colors import LogNorm
import os
import numpy as np
from scipy.interpolate import griddata

def read(filename):
    arg1s, arg2s, times = np.loadtxt(filename, delimiter = ',', unpack=True)
    data = {"arg1_list": arg1s, "arg2_list": arg2s, "time_list": times}
    return data

def plot_grid(data):
    l1 = data["arg1_list"]
    l2 = data["arg2_list"]
    t3 = np.array(data["time_list"])
    points = np.array([[a1, a2] for a1, a2 in zip(l1, l2)])
    x_grid, y_grid = np.mgrid[
        np.min(l1):np.max(l1):100j,
        np.min(l2):np.max(l2):100j]
    z_grid = griddata(points, t3, (x_grid, y_grid), method='nearest')[:-1,:-1]
    fig, ax = plt.subplots()
    plot = ax.pcolormesh(x_grid, y_grid, z_grid)
    fig.colorbar(plot, ax=ax)
    ax.set_aspect('equal', 'box')
    plt.show()

def plot_line(*data_args):
    fig, ax = plt.subplots()
    for data, label in data_args:
        ax.plot(np.log(data["arg1_list"]), np.log(data["time_list"]),
                label=label)
    ax.set_xlabel('natural log of first argument (x)')
    ax.set_ylabel('natural log of time elapsed in seconds')
    ax.set_title('log-log plot of time to compute gcd(x, 2x+1) given x')
    ax.set_aspect('equal', 'box')
    ax.legend(loc='upper left')
    plt.show()

#list_data = read(os.path.join("test_data", "list_euclid.txt"))
#string_data = read(os.path.join("test_data", "string_euclid.txt"))
#vector_data = read("vector_euclid.txt")
#plot_grid(string_data)

string_data = read(os.path.join("test_data", "string_euclid_1d.txt"))
list_data = read(os.path.join("test_data", "list_euclid_1d.txt"))
vector_data = read(os.path.join("test_data", "vector_euclid_1d.txt"))
plot_line(
    (string_data, "string impl"),
    (list_data, "list impl"),
    (vector_data, "vector impl"))