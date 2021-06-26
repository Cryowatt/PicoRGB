cmake_minimum_required(VERSION 3.13)

# Pull in SDK (must be before project)
include(pico_sdk_import.cmake)

project(pico_rgb C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# Initialize the SDK
pico_sdk_init()

# include(example_auto_set_url.cmake)
# # Add blink example
# add_subdirectory(blink)

# # Add hello world example
# add_subdirectory(hello_world)

add_compile_options(-Wall
        -Wno-format          # int != int32_t as far as the compiler is concerned because gcc has int32_t as long int
        -Wno-unused-function # we have some for the docs that aren't called
        -Wno-maybe-uninitialized
        )

add_executable(picorgb
        main.cpp
        )

# Pull in our pico_stdlib which pulls in commonly used features
target_include_directories(picorgb PRIVATE ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries(picorgb PRIVATE
        pico_stdlib
        tinyusb_device
        tinyusb_board
)

# create map/bin/hex file etc.
pico_add_extra_outputs(picorgb)

# # add url via pico_set_program_url
# example_auto_set_url(blink)