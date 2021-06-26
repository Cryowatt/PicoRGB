#pragma once
#include "Buffer.h"

class IShader {
public:
	virtual void Update(PixelBuffer& buffer) {}
};

class NoOpShader : public IShader {
public:
	static NoOpShader& getInstance()
	{
		static NoOpShader instance;
		return instance;
	}

	void Update(PixelBuffer& buffer) {

	}
};

class SolidShader : public IShader {
public:
	Pixel color;
	SolidShader(Pixel color) : color(color) {

	}

	void Update(PixelBuffer& buffer) {
		for (int i = 0; i < buffer.Length; i++) {
			buffer.Pixels[i] = color;
		}
	}
};

class IncrementingShader : public IShader {
public:
	Pixel color;

	void Update(PixelBuffer& buffer) {
		
		for (int i = 0; i < buffer.Length; i++) {
			buffer.Pixels[i] = color;
		}

		color.R++;
		color.G+=2;
		color.B+=3;
	}
};