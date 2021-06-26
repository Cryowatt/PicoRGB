import machine
import uasyncio

class Timer():
    def __init__(self, interval: int, function):
        self.id = id
        self.interval = interval
        self.function = function
        self.timer = machine.Timer()
        self.busy = uasyncio.Event()
    
    def run(self):
        self.timer.init(mode=machine.Timer.PERIODIC, period=int(self.interval), callback=self.run_function_no_overlap)
        
    def run_function_no_overlap(self, timer):
        if not self.busy.is_set():
            self.busy.set()
            self.function()
            self.busy.clear()
        else:
            print("Frame skip!")
            
    def cancel(self):
        self.timer.deinit()