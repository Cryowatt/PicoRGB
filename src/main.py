import engine
import time
import array
from machine import Pin
import rp2
import uctypes
from gradients import *
from shader import *
from channel import *
import gc
gc.enable()

class Renderer:
    def __init__(self):
        self.statemachines = [Renderer.create_statemachine(i) for i in range(8)]
        # picounicorn.init()
        # w = picounicorn.get_width()
        # h = picounicorn.get_height()
        # for x in range(w):
        #     for y in range(h):
        #         picounicorn.set_pixel(x, y, 0, 0, 0)
                
    def create_statemachine(i):
        # TODO: Map the pins to some shit
        statemachine = rp2.StateMachine(i, Renderer.ws2812, freq=8_000_000, sideset_base=Pin(16 + i))
        statemachine.active(1)
        return statemachine

    @rp2.asm_pio(sideset_init=rp2.PIO.OUT_LOW, out_shiftdir=rp2.PIO.SHIFT_LEFT, autopull=True, pull_thresh=24)
    def ws2812():
        T1 = 2
        T2 = 5
        T3 = 3
        wrap_target()
        label("bitloop")
        out(x, 1)               .side(0)    [T3 - 1]
        jmp(not_x, "do_zero")   .side(1)    [T1 - 1]
        jmp("bitloop")          .side(1)    [T2 - 1]
        label("do_zero")
        nop()                   .side(0)    [T2 - 1]
        wrap()
    
    def bytes_to_pixel(self, data, i) -> int:
        #return (data[i+3] << 24) + (data[i+2] << 16) + (data[i+1] << 8) + data[i]
        return (data[i] << 24) + (data[i+1] << 16) + (data[i+2] << 8)
        
    def render(self, engine: rgb.engine.Engine):
        for channel in zip(self.statemachines, engine.channels):
            #buffer = array.array("I", [self.bytes_to_pixel(channel[1].data, i) for i in range(0, len(channel[1].data)-4, 4)])
            #buffer = array.array("I", [self.bytes_to_pixel(channel[1].data, i) for i in range(0, len(channel[1].data)-4, 4)])
            #buffer = array.array("I", [0x00000000, 0xff000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0])
            #buffer = array.array("I", [self.bytes_to_pixel(channel[1].data, i) for i in range(0, len(channel[1].data), 3)])
            channel[0].put(channel[1].data)
            #time.sleep_us(50)
            #channel[0].put(channel[1].buffer, 9)
            #return
            #self.statemachines[0].put(channel.buffer, channel.length)
        # for y in range(picounicorn.get_height()):
        #     channel = engine.channels[y].buffer
            
        #     for x in range(min(len(channel), picounicorn.get_width())):
        #         r = (channel[x] >> 16) % 256
        #         g = (channel[x] >> 8) % 256
        #         b = (channel[x] >> 0) % 256
                # picounicorn.set_pixel(x, y, r, g, b)


rainbow = RainbowGradient(256)
grad = InterpolatedGradient((0.1, 0.1, 0.2), (0.0, 1.0, 1.0), 256)
grad2 = ThreePointInterpolatedGradient((1.00, 0.001, 1.0), (0.01, 0.0, 0.01), (0.001, 1.0, 1.0), 256)
channels = list(
    [Channel(9, HarmonicShader(grad2)),
    Channel(9, ChaseShader(grad, 0.5)), 
    Channel(9, ChaseShader(grad, -0.5)), 
    Channel(9, ChaseShader(rainbow, -1.0)), 
    Channel(64, ChaseShader(rainbow, 1.0)), 
    Channel(22, ChaseShader(rainbow, 1.0))])
engine = engine.Engine(Renderer(), channels, 60, 60, False)
#gc.disable()
try:
    engine.start()
    time.sleep(15.0)
    engine.stop()
finally:
    gc.enable()

# Example using PIO to drive a set of WS2812 LEDs.

# Start the StateMachine, it will wait for data on its FIFO.
# sm.active(1)

# # Display a pattern on the LEDs via an array of LED RGB values.
# ar = array.array("I", [0 for _ in range(NUM_LEDS)])

# # Cycle colours.
# for i in range(4 * NUM_LEDS):
#     for j in range(NUM_LEDS):
#         r = j * 100 // (NUM_LEDS - 1)
#         b = 100 - j * 100 // (NUM_LEDS - 1)
#         if j != i % NUM_LEDS:
#             r >>= 3
#             b >>= 3
#         ar[j] = r << 16 | b
#     sm.put(ar, 8)
#     time.sleep_ms(50)

# # Fade out.
# for i in range(24):
#     for j in range(NUM_LEDS):
#         ar[j] >>= 1
#     sm.put(ar, 8)
#     time.sleep_ms(50)
    