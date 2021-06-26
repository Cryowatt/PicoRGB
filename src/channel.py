from shader import Shader
from benchmark import benchmark
from colorbuffer import *

class Channel(ColorBuffer):
    def __init__(self, length: int, renderer: Shader):
        ColorBuffer.__init__(self, length)
        self.renderer = renderer
        self.renderTime = 0

    def reset(self):
        r = self.renderer
        r.reset(self)

    def update(self, delta):
        self.renderTime = benchmark(lambda: self.renderer.render(self, delta))