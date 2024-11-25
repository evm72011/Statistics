import numpy as np
from scipy.stats import norm
import matplotlib.pyplot as plt
import seaborn as sns; sns.set_theme()

n = 1000
x_mean, x_stddev = 2.0, 0.1
y_mean, y_stddev = 3.0, 0.2
x = np.random.normal(x_mean, x_stddev, n)
y = np.random.normal(y_mean, y_stddev, n)

s = x * y
s_mean, s_stddev = s.mean(), np.std(s, ddof=1)
print(s_mean, s_stddev)

p = len(s[(s > 5.9) & (s < 6.1)]) / n
print(p)

i = (np.arange(1, n + 1) - 0.5)  / n
z = norm.ppf(i)
q = z * s_stddev + s_mean
s.sort()

plt.scatter(q, s, color='green', alpha=0.5)

plt.show()
