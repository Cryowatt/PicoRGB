import time

def benchmark(function):
    start = time.perf_counter_ns()
    function()
    return time.perf_counter_ns() - start