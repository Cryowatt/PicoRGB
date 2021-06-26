from colorbuffer import *
import lib.colorsys as colorsys

def color_diff(a, b):
    return tuple(map(lambda i, j: i-j, a, b))

def color_add(a, b):
    #print(colorsys.yiq_to_rgb(*tuple(map(lambda i, j: i+j, a, b))))
    return tuple(map(lambda i, j: i+j, a, b))

def color_scale(color, v):
    #print(tuple(map(lambda i: i*v, color)))
    return tuple(map(lambda i: i*v, color))

class RainbowGradient(ColorBuffer):
    def __init__(self, steps: int):
        ColorBuffer.__init__(self, steps)
        i = 0
        for pixel in [colorsys.hls_to_rgb888(x / steps, 0.5, 1.0) for x in range(steps)]:
            bpixel = self.buffer[i]
            bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
            i += 1

class InterpolatedGradient(ColorBuffer):
    def __init__(self, start, end, steps: int):
        ColorBuffer.__init__(self, steps)
        i = 0
        startYiq = colorsys.rgb_to_hls(*start)
        delta = color_diff(colorsys.rgb_to_hls(*end), startYiq)
        for pixel in [colorsys.hls_to_rgb888(*color_add(startYiq, color_scale(delta, i / steps))) for i in range(steps)]:
            bpixel = self.buffer[i]
            bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
            i += 1

# I probably should make this generic n-point
class ThreePointInterpolatedGradient(ColorBuffer):
    def __init__(self, start, mid, end, steps: int):
        ColorBuffer.__init__(self, steps)
        i = 0
        startYiq = colorsys.rgb_to_hls(*start)
        delta = color_diff(colorsys.rgb_to_hls(*mid), startYiq)
        for pixel in [colorsys.hls_to_rgb888(*color_add(startYiq, color_scale(delta, i / (steps >> 1)))) for i in range(steps >> 1)]:
            bpixel = self.buffer[i]
            bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
            i += 1

        startYiq = colorsys.rgb_to_hls(*mid)
        delta = color_diff(colorsys.rgb_to_hls(*end), startYiq)
        for pixel in [colorsys.hls_to_rgb888(*color_add(startYiq, color_scale(delta, i / (steps >> 1)))) for i in range(steps >> 1)]:
            bpixel = self.buffer[i]
            bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
            i += 1