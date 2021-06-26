# CMakeList.txt : CMake project for CMakeProject1, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

# Add source to this project's executable.
add_executable (RGBWorkbench "Workbench.cpp" "Workbench.h")
target_link_libraries (RGBWorkbench LINK_PUBLIC PicoRGB)

# TODO: Add tests and install targets if needed.
