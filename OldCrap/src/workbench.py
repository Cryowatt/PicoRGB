import sys
sys.path.append("pylib")
from colored import fg, bg, attr
from datetime import datetime
from colorbuffer import *
from engine import *
from gradients import *
from shader import *
from channel import *

class DebugRenderer:
    reset = attr('reset')

    def __init__(self, outputEnabled = True):
        self.is_running = False
        self.outputEnabled = outputEnabled
        self.start_time = datetime.now()
        
    def renderPixel(self, led):
        escape = fg(f'#{led.r:02x}{led.g:02x}{led.b:02x}')
        return f"{escape}■"
        
    def printStrip(self, name, ledArray):
        array = "".join([self.renderPixel(led) for led in ledArray])
        print(f"{name}: {array}{self.reset}")

    def run(self, engine: Engine):
        self.is_running = True

        while self.is_running:
            if self.outputEnabled:
                self.render(engine)

    def render(self, engine: Engine):
        if self.outputEnabled:
            print(f'\033[{len(engine.channels)+2}A')
            for channel in engine.channels:
                self.printStrip(f"strip [{channel.renderTime}]\t", channel.buffer)
            print(engine.cycles / (datetime.now() - self.start_time).total_seconds())
    
    def stop(self):
        self.is_running = False

isRunning = True
rainbow = RainbowGradient(256)
grad = InterpolatedGradient((0.1, 0.1, 0.2), (0.0, 1.0, 1.0), 256)
grad2 = ThreePointInterpolatedGradient((1.00, 0.001, 1.0), (0.01, 0.0, 0.01), (0.001, 1.0, 1.0), 256)
channels = list(
    [Channel(50, ChaseShader(grad2, 1.0)),
    Channel(9, ChaseShader(grad, 0.5)), 
    Channel(9, ChaseShader(grad, -0.5)), 
    Channel(9, ChaseShader(rainbow, -1.0)), 
    Channel(9, RandomRampShader(grad2)), 
    Channel(9, HarmonicShader(grad2)), 
    Channel(64, RandomRampShader(grad2)), 
    Channel(64, HarmonicShader(grad2))])
engine = Engine(DebugRenderer(True), channels, 60, 60, False)
reset = attr('reset')
start_time = datetime.now()

def stop():
    global isRunning
    isRunning = False
    engine.stop()

def run(engine: Engine):
    try:
        engine.run()
    except Exception as e:
        stop()
        raise e

try:
    for channel in engine.channels:
        print()
    engine.start()
    input()
finally:
    stop()
