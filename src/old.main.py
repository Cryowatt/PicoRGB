from colored import fg, bg, attr
import random
import math
import lib.colorsystems as colorsys
import numpy as np
import threading
from datetime import timedelta, datetime
reset = attr('reset')

def clamp(n, smallest, largest): return max(smallest, min(n, largest))

class Spinner():
    ledPerSecond = 3

    def __init__(self, length = 9, angle = 0):
        self.length = length
        self.angle = angle
        self.color1 = np.array([0.2, 0.0, 0.95])
        self.color1rgb = (self.color1 * 255)    
        self.color1hls = np.array(colorsys.rgb_to_yiq(*self.color1))
        self.color2 = np.array([0.2, 0.95, 0.95])
        self.color2rgb = (self.color2 * 255)
        self.color2hls = np.array(colorsys.rgb_to_yiq(*self.color2))

    def render(self, delta):
        self.angle = (self.angle + delta.total_seconds() * self.ledPerSecond) % self.length
        fraction = self.angle % 1
        borderPixel = (self.color1hls * fraction) + (self.color2hls * (1 - fraction))
        middle = np.array(colorsys.yiq_to_rgb(*borderPixel)) * 255

        angle2 = (self.angle + (self.length / 2)) % self.length
        fraction = angle2 % 1
        borderPixel = (self.color2hls * fraction) + (self.color1hls * (1 - fraction))
        middle2 = np.array(colorsys.yiq_to_rgb(*borderPixel)) * 255
        # def foo(led):
        #     if(self.angle < angle2):
        #         if(led == int(self.angle)):
        #             return middle
        #         elif(led == int(angle2)):
        #             return middle2
        #         elif(led > self.angle and led < angle2):
        #             return self.color2rgb
        #         else:
        #             return self.color1rgb
        #     else:
        #         if(led == int(self.angle)):
        #             return middle
        #         elif(led == int(angle2)):
        #             return middle2
        #         elif(led > angle2 and led < self.angle):
        #             return self.color1rgb
        #         else:
        #             return self.color2rgb
        #     return x
        return [self.color1rgb for a in range(self.length)]

def renderPixel(led):
    escape = fg(f"#{led.astype(np.uint8).tobytes().hex()}")
    return f"{escape}■"

def printStrip(name, ledArray):
    array = "".join([renderPixel(led) for led in ledArray])
    print(f"{name}: {array}{reset}")


print("\x1b[2J")
lastUpdate = datetime.now()
ext = Spinner()
top0 = Spinner(angle=0.00)
top1 = Spinner(angle=3)
top2 = Spinner(angle=6)
frt0 = Spinner(angle=3)
frt1 = Spinner(angle=6)
cpu = Spinner(length = 20)
gpu = Spinner(length = 20)
dist = Spinner(length = 22)
start = datetime.now()
updates = 0

while((datetime.now() - start).total_seconds() < 10):
    now = datetime.now()
    delta = now - lastUpdate
    lastUpdate = now

    printStrip("Ext ", ext.render(delta))
    printStrip("Top0", top0.render(delta))
    printStrip("Top1", top1.render(delta))
    printStrip("Top2", top2.render(delta))
    printStrip("Frt0", frt0.render(delta))
    printStrip("Frt1", frt1.render(delta))
    printStrip("CPU ", cpu.render(delta))
    printStrip("GPU ", gpu.render(delta))
    printStrip("Dist", dist.render(delta))
    print("\x1b[10A")
    updates = updates + 1

print(F"Updates per second: {(updates / 10.0)}")