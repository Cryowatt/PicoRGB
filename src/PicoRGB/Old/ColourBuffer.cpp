#include "ColourBuffer.hpp" 

// Define the constructor.
ColorBuffer::ColorBuffer(int length) {
    this->pData = new ColourData[length];
    this->length = length;
}

// Define the destructor.
ColorBuffer::~ColorBuffer() {
    delete[] this->pData;
}