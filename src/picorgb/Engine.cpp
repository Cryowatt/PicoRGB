#include <cmath>
#include "Engine.h"
#include <iostream>

using namespace std;
//#include "ColourBuffer.h"

//ColorBuffer::ColorBuffer(int length) : data(new Color[length]),
//                                       length(length)
//{
//}
//
//ColorBuffer::~ColorBuffer()
//{
//    delete[] this->data;
//}
//
//Channel::Channel(Shader &shader, int length) : shader(shader),
//                                               pixels(new ColorBuffer(length)),
//                                               length(length)
//{
//}
//
//Channel::~Channel()
//{
//    delete this->pixels;
//}
//
//void Channel::Update()
//{
//    //
//}
//
//ChaseShader::ChaseShader(ColorBuffer &source, float cyclesPerSecond) : chase(source),
//                                                                       cyclesPerSecond(cyclesPerSecond)
//{
//}
//
//void ChaseShader::Reset(Channel &channel)
//{
//    this->position = 0.0f;
//}
//
//void ChaseShader::Render(Channel &channel, float delta)
//{
//    this->position += (channel.length * this->cyclesPerSecond) * delta;
//    const float period = this->chase.length / channel.length;
//    float p;
//    Color *cb = channel.pixels->data;
//
//    for (int i = 0; i < channel.length; i++)
//    {
//        int bi = static_cast<int>(p) % this->chase.length;
//        *(cb++) = this->chase.data[bi];
//        p += period;
//    }
//
//    // self.position += (len(self.chase) * self.cycles_per_second) * delta
//    // period = len(self.chase) / channel.length
//    // p = self.position
//    // for i in range(0, channel.length):
//    //     if abs(p) >= self.chase.length:
//    //         p %= self.chase.length
//    //     # p = int((self.position + (i * (len(self.chase) / channel.length))) % self.chase.length))
//    //     # print(p)
//    //     # cpixel = self.chase.buffer[int(p)]
//    //     channel.buffer[i].data = self.chase.buffer[int(p)].data
//    //     # bpixel = channel.buffer[i]
//    //     # bpixel.data = cpixel.data
//    //     p += period
//}
//
//// class ChaseShader
//// {
//// public:
////     virtual Reset(Channel& channel)
////     virtual Render(Channel& channel);
//
////     def __init__(self, source: ColorBuffer, cycles_per_second: float):
////         self.chase = source
////         # self.chase = rgb.colorbuffer.ColorBuffer(256)
////         # i = 0
////         # for pixel in [colorsys.hls_to_rgb888(x / 256.0, 0.5, 1.0) for x in range(256)]:
////         #     bpixel = self.chase.buffer[i]
////         #     bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
////         #     i += 1
////         self.position = 0.0
////         self.cycles_per_second = cycles_per_second
//
////     def reset(self, channel):
////         self.position = 0.0
//
////     def render(self, channel, delta: float):
////         self.position += (len(self.chase) * self.cycles_per_second) * delta
////         period = len(self.chase) / channel.length
////         p = self.position
////         for i in range(0, channel.length):
////             if abs(p) >= self.chase.length:
////                 p %= self.chase.length
////             # p = int((self.position + (i * (len(self.chase) / channel.length))) % self.chase.length))
////             # print(p)
////             # cpixel = self.chase.buffer[int(p)]
////             channel.buffer[i].data = self.chase.buffer[int(p)].data
////             # bpixel = channel.buffer[i]
////             # bpixel.data = cpixel.data
////             p += period
//// }

//Engine::Engine()
//{
//}

//void Engine::Update()
//{
//    //
//}

void Engine::Update() {
    for (int i = 0; i < MAX_CHANNELS; i++) {
        this->Update(i);
    }
}

void Engine::Update(int channelId) {
    this->channels[channelId].Update();
}
