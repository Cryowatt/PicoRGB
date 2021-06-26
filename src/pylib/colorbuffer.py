import ctypes

class ColorData(ctypes.Structure):
    _fields_ = [
        ("g", ctypes.c_uint8), 
        ("r", ctypes.c_uint8),
        ("b", ctypes.c_uint8)
        ]

class Color(ctypes.Union):
    _fields_ = [
        ("data", ctypes.c_uint32), 
        ("color", ColorData),
        ]
    
    @property
    def r(self):
        return self.color.r

    @r.setter
    def r(self, value: int):
        self.color.r = value

    @property
    def g(self):
        return self.color.g

    @g.setter
    def g(self, value: int):
        self.color.g = value

    @property
    def b(self):
        return self.color.b
        
    @b.setter
    def b(self, value: int):
        self.color.b = value


class ColorBuffer:
    def __init__(self, length: int) -> None:
        arrayType = Color * length
        self.buffer = arrayType()
        self.length = length

    def __len__(self):
        return self.length