import time

def benchmark(function):
    start = time.ticks_us()
    function()
    return time.ticks_diff(time.ticks_us(), start)