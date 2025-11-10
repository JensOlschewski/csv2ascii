import random

num_points = 500
with open("points.csv", "w") as f:
    for i in range(1, num_points + 1):
        x = round(random.uniform(0, 100), 2)
        y = round(random.uniform(0, 100), 2)
        z = round(random.uniform(0, 100), 2)
        f.write(f"{x},{y},{z}\n")
