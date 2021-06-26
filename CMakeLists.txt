# CMakeList.txt : Top-level CMake project file, do global configuration
# and include sub-projects here.
#
cmake_minimum_required (VERSION 3.8)

project ("PicoRGB")

# Include sub-projects.
add_subdirectory ("src/Workbench")
add_subdirectory ("src/PicoRGB")
#add_subdirectory ("src/Image")
