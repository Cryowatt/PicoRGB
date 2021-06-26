# CMakeList.txt : CMake project for CMakeProject1, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

# Add source to this project's executable.
add_library(PicoRGB "Engine.cpp" "Engine.h" "Channel.h" "Shaders.h" "Buffer.h")
target_include_directories (PicoRGB PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})


# TODO: Add tests and install targets if needed.
