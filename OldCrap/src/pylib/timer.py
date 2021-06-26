import threading
import asyncio

class Timer():
    def __init__(self, interval: int, function):
        self.interval = interval / 1000.0
        self.function = function
        self.event = threading.Event()
        self.thread = threading.Thread(target = self.handler)
        
    def run(self):
        self.thread.start()

    def handler(self):
        asyncio.run(self.handler_coroutine())
    
    async def handler_coroutine(self):
        # while not self.event.wait(self.interval):
        while not self.event.is_set():
            delay = asyncio.sleep(self.interval)
            try:
                self.function()
            finally:
                await delay

    def cancel(self):
        self.event.set()