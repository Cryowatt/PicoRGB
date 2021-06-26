#pragma once
#include "Shaders.h"
#include "Buffer.h"

class Channel {
public:
	PixelBuffer* buffer = new PixelBuffer(0);
	int Length = 0; // TODO: Do I need this still? it's in the buffer
	IShader* Shader = &NoOpShader::getInstance();

	void Resize(int length) {
		delete this->buffer;
		this->Length = length;
		this->buffer = new PixelBuffer(length);
	}

	void Update() {
		this->Shader->Update(*this->buffer);
	}
};