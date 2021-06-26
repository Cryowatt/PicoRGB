#pragma once
// import array
// import uctypes

// color_type = {
//     "g": (0x3 | uctypes.UINT8),
//     "r": (0x2 | uctypes.UINT8),
//     "b": (0x1 | uctypes.UINT8),
//     "data": (0x0 | uctypes.UINT32)
//     }

// class ColorBuffer:
//     def __init__(self, length: int) -> None:
//         buffer_type = (uctypes.ARRAY, length, color_type)
//         self.data = array.array("I", [0] * length)
//         self.buffer = uctypes.struct(uctypes.addressof(self.data), buffer_type)
//         self.length = length

//     def __len__(self):
//         return self.length

typedef char byte;
typedef int A8B8R8G8;

struct ColourFormat {
    byte Alpha;
    byte Blue;
    byte Red;
    byte Green;
};

union ColourData {
    ColourFormat Colour;
    A8B8R8G8 data;
};

class ColorBuffer {    
public:
    ColorBuffer(int length);
    ~ColorBuffer();
private:
    ColourData* pData;
    int length;
};
