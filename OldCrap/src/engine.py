from benchmark import benchmark
from timer import Timer
import time
from channel import Channel
import benchmark

class Engine:

    def __init__(self, renderer, channels, engine_fps, render_fps, benchmarkEnabled=False):
       
        self.channels = channels
        self.cycles = 0
        self.renderer = renderer
        self.channel_perf = []
        if benchmarkEnabled:
            update_target = lambda: print(benchmark.benchmark(lambda: self.update(1.0 / engine_fps))) 
        else: 
            update_target = lambda: self.update(1.0 / engine_fps)
        self.update_timer = Timer(interval=int(1000.0 / engine_fps), function=update_target)
        self.render_timer = Timer(interval=int(1000.0 / render_fps), function=lambda: self.render())
    
    def start(self):
        self.update_timer.run()
        self.render_timer.run()
    
    def update(self, delta):
        for channel in self.channels:
            channel.update(delta)
        self.cycles += 1
    
    def render(self):
        self.renderer.render(self)
        #duration = time.ticks_diff(time.ticks_us(), start)
        #print(duration)
    
    def stop(self):
        self.update_timer.cancel()
        self.render_timer.cancel()