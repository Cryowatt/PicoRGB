# CMakeList.txt : CMake project for CMakeProject1, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)
SET(PICO_SDK_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../../pico-sdk")

include(pico_sdk_import.cmake)
project(PicoRGB_Image C CXX ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()

add_compile_options(-Wall
        -Wno-format          # int != int32_t as far as the compiler is concerned because gcc has int32_t as long int
        -Wno-unused-function # we have some for the docs that aren't called
        -Wno-maybe-uninitialized
        )

set(FAMILY rp2040)
set(BOARD pico_sdk)
set(PROJECT PicoRGB_Image)

#add_subdirectory("../PicoRGB")
# Add source to this project's executable.
add_executable (${PROJECT} "Main.cpp" "Main.h")
target_include_directories(${PROJECT} PRIVATE ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries (${PROJECT} LINK_PUBLIC PicoRGB)


# create map/bin/hex file etc.
pico_add_extra_outputs(${PROJECT})

# TODO: Add tests and install targets if needed.
# Pull in SDK (must be before project)

#project(picorgb C CXX ASM)
#set(CMAKE_C_STANDARD 11)
#set(CMAKE_CXX_STANDARD 17)

# Initialize the SDK


#add_compile_options(-Wall
        #-Wno-format          # int != int32_t as far as the compiler is concerned because gcc has int32_t as long int
        #-Wno-unused-function # we have some for the docs that aren't called
        #-Wno-maybe-uninitialized
        #)

#set(FAMILY rp2040)
#set(BOARD pico_sdk)
#set(PROJECT picorgb)
#include(${PICO_TINYUSB_PATH}/hw/bsp/${FAMILY}/family.cmake)

#add_executable(${PROJECT})

#target_sources(${PROJECT} PUBLIC
        #${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
        #${CMAKE_CURRENT_SOURCE_DIR}/Engine.cpp
        #${CMAKE_CURRENT_SOURCE_DIR}/usb_descriptors.c
        #${CMAKE_CURRENT_SOURCE_DIR}/ColourBuffer.cpp
        #)

# Pull in our pico_stdlib which pulls in commonly used features
#target_include_directories(${PROJECT} PRIVATE ${CMAKE_CURRENT_LIST_DIR})
#target_link_libraries(${PROJECT} PRIVATE
        #pico_stdlib
        #tinyusb_device
        #tinyusb_board
#)

# create map/bin/hex file etc.
#pico_add_extra_outputs(${PROJECT})


################################

# include(${PICO_TINYUSB_PATH}/hw/bsp/${FAMILY}/family.cmake)

# # gets PROJECT name for the example (e.g. <BOARD>-<DIR_NAME>)
# family_get_project_name(PROJECT ${CMAKE_CURRENT_LIST_DIR})

# project(${PROJECT})

# # Checks this example is valid for the family and initializes the project
# family_initialize_project(${PROJECT} ${CMAKE_CURRENT_LIST_DIR})

# add_executable(${PROJECT})

# # Example source
# target_sources(${PROJECT} PUBLIC
#         ${CMAKE_CURRENT_SOURCE_DIR}/src/main.c
#         ${CMAKE_CURRENT_SOURCE_DIR}/src/usb_descriptors.c
#         )

# # Example include
# target_include_directories(${PROJECT} PUBLIC
#         ${CMAKE_CURRENT_SOURCE_DIR}/src
#         )

# # Configure compilation flags and libraries for the example... see the corresponding function
# # in hw/bsp/FAMILY/family.cmake for details.
# family_configure_device_example(${PROJECT})