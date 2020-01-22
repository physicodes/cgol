#!/usr/bin/env python
# coding: utf-8

# In[1]:


from pathlib import Path

notebook_dir = Path('data')
data_file_paths = [p for p in notebook_dir.iterdir()]


# In[2]:


from collections import OrderedDict
import numpy as np
import re

data_dict = dict()

for path in data_file_paths:
    matched = re.search('data/(.+?).csv', str(path))
    frac = matched.group(1)
    data = np.loadtxt(path, dtype=int, delimiter=',')
    data_avg = np.mean(data, axis=0)/(100*100)
    data_std = np.std(data, axis=0)/(100*100)
    data_dict[frac] = (data_avg, data_std)
    
sorted_data_dict = OrderedDict(sorted(data_dict.items(), key=lambda t: float(t[0])))
sorted_data_dict


# In[3]:


print("Read in fractions:")
_ = [print(frac) for frac in sorted_data_dict]


# In[4]:


get_ipython().run_line_magic('matplotlib', 'inline')
import matplotlib.pyplot as plt
from itertools import cycle

fig, ax = plt.subplots(figsize=(15, 15))

colors = cycle(['C'+str(n) for n in range(9)])

for frac, color in zip([f for f in sorted_data_dict if len(f)<4], colors):
    y, yerr = sorted_data_dict.get(frac)
    x = np.arange(len(y))
    y_upper = y + yerr
    y_lower = y - yerr
    ax.plot(x, y, label=frac, color=color)
    ax.fill_between(x, y_upper, y_lower, facecolor=color, alpha=0.2)

ax.set_xlabel("Iterations")
ax.set_ylabel("Fraction Alive")
ax.legend()

fig.savefig("full_results.png", dpi=150)


# In[5]:


ax.set_ylim(0, 0.2)

fig


# In[6]:


ax.set_xlim(0, 250)

fig


# In[8]:


fig, ax = plt.subplots(figsize=(15, 15))

for frac, color in zip([f for f in sorted_data_dict if 0 < float(f) < 0.3], colors):
    y, yerr = sorted_data_dict.get(frac)
    x = np.arange(len(y))
    y_upper = y + yerr
    y_lower = y - yerr
    ax.plot(x, y, label=frac, color=color)
    ax.fill_between(x, y_upper, y_lower, facecolor=color, alpha=0.2)

ax.set_xlabel("Iterations")
ax.set_ylabel("Fraction Alive")
ax.legend()

fig.savefig("full_results.png", dpi=150)


# In[ ]:





# In[ ]:




