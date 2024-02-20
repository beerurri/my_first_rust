import numpy as np
from matplotlib import pyplot as plt

source = np.loadtxt('dump.csv', delimiter=',')[:20000]

plt.imshow(source, cmap='plasma', origin='lower', aspect='auto', interpolation='nearest')
plt.show()