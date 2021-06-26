#pragma once

struct Pixel {
    unsigned char R;
    unsigned char G;
    unsigned char B;
    unsigned char A;
};

union PixelData {
    Pixel Color;
    int Data;
};

class PixelBuffer {
public:
    PixelBuffer(int length) :
        Length(length), Pixels(new Pixel[length]) {
        Data = (PixelData*)Pixels;
    }

    ~PixelBuffer() {
        delete Pixels;
    }

    Pixel* Pixels;
    PixelData* Data;
    int	Length;
};