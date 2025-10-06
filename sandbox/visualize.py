import json
import matplotlib.pyplot as plt

with open("resource_usage.json", "r") as f:
    data = json.load(f)

timestamps = [d["timestamp"] for d in data]
cpu_usage = [d["cpu_usage"] for d in data]
memory_usage = [d["memory_usage"] for d in data]

fig, ax1 = plt.subplots()

color = 'tab:red'
ax1.set_xlabel('Time (s)')
ax1.set_ylabel('CPU Usage (s)', color=color)
ax1.plot(timestamps, cpu_usage, color=color)
ax1.tick_params(axis='y', labelcolor=color)

ax2 = ax1.twinx()
color = 'tab:blue'
ax2.set_ylabel('Memory Usage (bytes)', color=color)
ax2.plot(timestamps, memory_usage, color=color)
ax2.tick_params(axis='y', labelcolor=color)

fig.tight_layout()
plt.savefig("resource_usage.png")

