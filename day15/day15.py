from dataclasses import dataclass
import numpy as np
from scipy.spatial.distance import cdist
import os
import re
datas = """Sensor at x = 2, y = 18: closest beacon is at x = -2, y = 15
Sensor at x = 9, y = 16: closest beacon is at x = 10, y = 16
Sensor at x = 13, y = 2: closest beacon is at x = 15, y = 3
Sensor at x = 12, y = 14: closest beacon is at x = 10, y = 16
Sensor at x = 10, y = 20: closest beacon is at x = 10, y = 16
Sensor at x = 14, y = 17: closest beacon is at x = 10, y = 16
Sensor at x = 8, y = 7: closest beacon is at x = 2, y = 10
Sensor at x = 2, y = 0: closest beacon is at x = 2, y = 10
Sensor at x = 0, y = 11: closest beacon is at x = 2, y = 10
Sensor at x = 20, y = 14: closest beacon is at x = 25, y = 17
Sensor at x = 17, y = 20: closest beacon is at x = 21, y = 22
Sensor at x = 16, y = 7: closest beacon is at x = 15, y = 3
Sensor at x = 14, y = 3: closest beacon is at x = 15, y = 3
Sensor at x = 20, y = 1: closest beacon is at x = 15, y = 3"""


@dataclass
class Point:
    x: float
    y: float


class Sensor:
    p: Point


class Beacon:
    p: Point


for line in datas.splitlines():
    print(line)
    m = re.search(
        r'x = (.*?), y = (.*?):.*?x = (.*?), y = (.*)', line)

    print("===" + m.group(1))
    print("===" + m.group(2))
    print("===" + m.group(3))
    print("===" + m.group(4))
    print(30*"=")

"""
sx0, sy0 = 10, 10
bx0 = 20
by0 = 30

matrix = np.ones((50, 50))
print(matrix.shape)
signal = np.zeros((50, 50))
signal[2][1] = 1
# assuming each dimension includes 500 points, from 0 to 500, step 1
x, y = np.ogrid[0:500, 0:500]
#distances = np.sqrt((x-sx0)**2+(y-sy0)**2) < 100
distances = cdist(matrix, signal, 'cityblock')
print(distances)

def f(x,y):
    return x+y

np.indices((2,3))
"""


def sans_retour():
    return 42


print(sans_retour())
