import matplotlib.pyplot as plt
import numpy as np

t = np.linspace(-.2, .5, 1000)
plt.plot(t, np.cos(t), color='k');

dx = .2
xpoints = np.arange(0, .5 + dx/2, dx)
plt.plot(xpoints, np.cos(xpoints), 'ko')

x1 = dx
x2 = x1 + dx
x0 = x1 - dx
forward_slope = (np.cos(x2) - np.cos(x1))/dx
backward_slope = (np.cos(x1) - np.cos(x0))/dx
average_slope = (forward_slope + backward_slope)/2
fx1 = np.cos(x1)
fx0 = np.cos(x0)
forward = lambda x: (x-x1) * forward_slope + fx1
backward = lambda x: (x-x1) * backward_slope + fx1
average = lambda x: (x-x0) * average_slope + fx0
average_aligned = lambda x: (x-x1) * average_slope + fx1



plt.plot(t, forward(t), "r:")
plt.plot(t, backward(t), "b:")
plt.plot(t, average(t), "g:")
plt.plot(t, average_aligned(t), "g--")
plt.show()

