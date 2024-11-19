from scipy.stats import binom
n, p = 20, 0.4
stats = binom.stats(n, p, moments='mvsk')
print(stats)
p = 1 - binom.cdf(10, n, p, loc=0)
print(p)

