// CMakeProject1.cpp : Defines the entry point for the application.
//

#include <iostream>
#include "Workbench.hpp"
#include "Engine.hpp"
#include "Buffer.hpp"
#include <thread>         // std::this_thread::sleep_for
#include <chrono>         // std::chrono::seconds

using namespace std;
const string RESET = "\x1b[0m";

class DebugRenderer : public IRenderer {

public:
	void Render(Channel& channel) {
		cout << "CHANNEL [";
		for (int i = 0; i < channel.Length; i++) {
			cout << "\x1b[38;2;" << (int)channel.buffer->Pixels[i].R << ";" << (int)channel.buffer->Pixels[i].G << ";" << (int)channel.buffer->Pixels[i].B << "m#";
		}
		cout << RESET << "]" << channel.Length << endl;
	}
};

int main()
{
	DebugRenderer renderer;
	Engine engine(renderer);
	SolidShader solidShader(Pixel{ 100,200,30,4 });
	IncrementingShader incShader;
	PixelBuffer* rainbow  = PixelBuffer::Rainbow();
	ChaseShader chaseShader(rainbow);
	engine.Update();
	engine.Render();
	cout << "Configuring..." << endl;
	EngineConfiguration config;
	config.Channels[0].Length = 31;
	config.Channels[0].Shader = &solidShader;
	config.Channels[1].Length = 40;
	config.Channels[1].Shader = &incShader;
	config.Channels[2].Length = 40;
	config.Channels[2].Shader = &chaseShader;
	engine.Configure(config);

	cout << "\x1b[s\x1b[?25l";
	//for (int i = 0; i < 512; i++) {
	while(1) {
		cout << "\x1b[u";
		engine.Update();
		engine.Render();
		//std::this_thread::sleep_for(std::chrono::milliseconds(1));
	}
	cout << RESET;
	
	return 0;
}
