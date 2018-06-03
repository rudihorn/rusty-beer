import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np


data = np.genfromtxt('output.csv', delimiter=',')
time = data[:,0]/60

fig, ax1 = plt.subplots()

c1 = 'tab:red'
ax1.set_xlabel("time")
ax1.set_ylabel("Temperature", color=c1)
ax1.tick_params(axis='y', labelcolor=c1)
ax1.plot(time, data[:,2], color=c1)

c2 = 'tab:blue'
ax2 = ax1.twinx()
ax2.set_ylabel('duty', color=c2) 
ax2.plot(time, data[:,1], color=c2);
ax2.tick_params(axis='y', labelcolor=c2)

fig.tight_layout()

plt.show()
