from array import array
import colorsys as colorsys
from colorbuffer import *
import ctypes
import random
import math


class Shader:
    def __init__(self):
        pass

    def reset(self, channel):
        channel.targetbuffer = array('I', [0 for i in range(channel.length)])

    def render(self, channel, delta):
        pass

    def clampint(v, lower, upper):
        return int(max(lower, min(v, upper)))


class ChaseShader(Shader):
    def __init__(self, source: ColorBuffer, cycles_per_second: float):
        self.chase = source
        # self.chase = rgb.colorbuffer.ColorBuffer(256)
        # i = 0
        # for pixel in [colorsys.hls_to_rgb888(x / 256.0, 0.5, 1.0) for x in range(256)]:
        #     bpixel = self.chase.buffer[i]
        #     bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
        #     i += 1
        self.position = 0.0
        self.cycles_per_second = cycles_per_second

    def reset(self, channel):
        self.position = 0.0

    def render(self, channel, delta: float):
        self.position += (len(self.chase) * self.cycles_per_second) * delta
        period = len(self.chase) / channel.length
        p = self.position
        for i in range(0, channel.length):
            if abs(p) >= self.chase.length:
                p %= self.chase.length
            # p = int((self.position + (i * (len(self.chase) / channel.length))) % self.chase.length))
            # print(p)
            # cpixel = self.chase.buffer[int(p)]
            channel.buffer[i].data = self.chase.buffer[int(p)].data
            # bpixel = channel.buffer[i]
            # bpixel.data = cpixel.data
            p += period


class TestShader(Shader):
    def __init__(self):
        self.position = 0

    def reset(self, channel):
        self.position = 0

    def render(self, channel, delta: float):
        for i in range(0, channel.length):
            p = i + self.position
            bpixel = channel.buffer[i]
            bpixel.r = 255 if p % 3 == 0 else 0
            bpixel.g = 255 if (p + 1) % 3 == 0 else 0
            bpixel.b = 255 if (p + 2) % 3 == 0 else 0
        self.position += 1

class HarmonicShader(Shader):
    def __init__(self, colors: ColorBuffer):
        self.colors = colors
        # self.harmonics = [(math.pi * 2, 0.9), (math.pi * 4, -0.44), (math.pi * 6, 1.31), (math.pi * 8, -3.1)]
        self.harmonics = [(math.pi * 2, 0.9), (math.pi * 4, -0.44), (math.pi * 6, 1.31), (math.pi * 8, -3.1)]
        self.position = 0.0

    def reset(self, channel):
        self.position = 0.0

    def render(self, channel, delta: float):
        self.position += delta
        for i in range(channel.length):
            led_theta = (i / channel.length)
            lv = 0
            for h in self.harmonics:
                lv += (math.sin((h[0] * led_theta) + (self.position * h[1]))) + 1
            # lv = sum([math.sin((h[0] * led_theta) + (self.position * h[1])) for h in self.harmonics])
            channel.buffer[i].data = self.colors.buffer[int((lv / 8) * (self.colors.length - 1))].data 
            #channel.buffer[i].data = self.colors.buffer[Shader.clampint((lv / 4) * self.colors.length, 0, self.colors.length - 1)].data 
        # for i, lv in enumerate([sum([math.sin((h[0] * (i / channel.length)) + (self.position * h[1])) for h in self.harmonics]) for i in range(channel.length)]):
        #     channel.buffer[i].data = self.colors.buffer[Shader.clampint((lv / 4) * self.colors.length, 0, self.colors.length - 1)].data
        # for h in self.harmonics:
        #     math.sin(h[0] * )
        # math.sin((math.pi))

# This is crap, rename to HarmoicShader: sin((harmonic*positional)+(tz*velocity)). Probably three components and make it faster
class RandomRampShader(Shader):
    def __init__(self, colors: ColorBuffer):
        self.colors = colors

    def reset(self, channel):
        pass

    def render(self, channel, delta: float):
        av = random.uniform(0.0, 1.0)
        # av = random.randint(0, self.colors.length - 1)
        channel.buffer[0].data = self.colors.buffer[Shader.clampint(
            av * self.colors.length, 0, self.colors.length - 1)].data
        bv = random.uniform(0.0, 1.0)
        # bv = random.randint(0, self.colors.length - 1)
        b = channel.length >> 1
        channel.buffer[b].data = self.colors.buffer[Shader.clampint(
            bv * self.colors.length, 0, self.colors.length - 1)].data

        self.setmidpointpixel(channel, 0, b, av, bv, 0.25)
        self.setmidpointpixel(channel, b, channel.length, bv, av, 0.25)
        # quit()
        # random.randrange(channel.buffer.length)
        # for i in range(0, channel.length):
        #     p = i + self.position
        #     bpixel = channel.buffer[i]
        #     bpixel.r = 255 if p % 3 == 0 else 0
        #     bpixel.g = 255 if (p + 1) % 3 == 0 else 0
        #     bpixel.b = 255 if (p + 2) % 3 == 0 else 0
        # self.position += 1

    def setmidpointpixel(self, channel, a, b, av, bv, magnitude):
        r = ((av + bv) / 2) + random.uniform(-magnitude, magnitude)
        m = (a + b) >> 1
        channel.buffer[m].data = self.colors.buffer[Shader.clampint(
            r * self.colors.length, 0, self.colors.length - 1)].data
        if m - a > 1:
            self.setmidpointpixel(channel, a, m, av, r, magnitude / 2)
        if b - m > 1:
            self.setmidpointpixel(channel, m, b, r, bv, magnitude / 2)
