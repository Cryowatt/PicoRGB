// CMakeProject1.cpp : Defines the entry point for the application.
//

#include <iostream>
#include "Workbench.h"
#include "Engine.h"
#include "Buffer.h"

using namespace std;
const string RESET = "\x1b[0m";

class DebugRenderer : public IRenderer {

public:
	void Render(Channel& channel) {
		cout << "CHANNEL [";
		for (int i = 0; i < channel.Length; i++) {
			cout << "\x1b[38;2;" << (int)channel.buffer->Pixels[i].R << ";" << (int)channel.buffer->Pixels[i].G << ";" << (int)channel.buffer->Pixels[i].B << "m#";
		}
		cout << "]" << channel.Length << RESET << endl;
	}
};

int main()
{
	DebugRenderer renderer;
	Engine engine(renderer);
	SolidShader solidShader(Pixel{ 100,200,30,4 });
	IncrementingShader incShader;
	engine.Update();
	engine.Render();
	cout << "Configuring..." << endl;
	EngineConfiguration config;
	config.Channels[0].Length = 8;
	config.Channels[0].Shader = &solidShader;
	config.Channels[1].Length = 16;
	config.Channels[1].Shader = &incShader;
	engine.Configure(config);

	cout << "\x1b[s\x1b[?25l";
	for (int i = 0; i < 512; i++) {
		cout << "\x1b[u";
		engine.Update();
		engine.Render();
	}
	cout << RESET;
	
	return 0;
}
