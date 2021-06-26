#include <vector>
#include <array>
#include "Channel.h"
#include "Buffer.h"

const size_t MAX_CHANNELS = 8;

class IRenderer {
public:
	virtual void Render(Channel& channel) {}
};

struct ChannelConfiguration {
public:
	ChannelConfiguration() : Length(0), Shader(&NoOpShader::getInstance()) {

	}
	int Length;
	IShader* Shader;
};

struct EngineConfiguration {
public:
	ChannelConfiguration Channels[MAX_CHANNELS];
};

class Engine
{
private:
	IRenderer& renderer;
	Channel channels[MAX_CHANNELS];

public:
    Engine(IRenderer& renderer) :
		renderer(renderer)
	{

	}

	void Configure(EngineConfiguration& configuration) {
		for (int i = 0; i < MAX_CHANNELS; i++) {
			if (configuration.Channels[i].Length > 0) {
				this->channels[i].Resize(configuration.Channels[i].Length);
				this->channels[i].Shader = configuration.Channels[i].Shader;
			}
		}
	}

	void Update();
	void Update(int channelId);
	void Render() {
		for (int i = 0; i < MAX_CHANNELS; i++) {
			this->Render(i);
		}
	}
	void Render(int channelId) {
		this->renderer.Render(this->channels[channelId]);
	}
};

