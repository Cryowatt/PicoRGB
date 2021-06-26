cmake_minimum_required(VERSION 3.13)

# Pull in SDK (must be before project)
include(pico_sdk_import.cmake)
set(FAMILY rp2040)
set(BOARD pico_sdk)

# use BOARD-Directory name for project id
get_filename_component(PROJECT ${CMAKE_CURRENT_SOURCE_DIR} NAME)
set(PROJECT ${BOARD}-${PROJECT})

# TOP is absolute path to root directory of TinyUSB git repo
set(TOP "../../..")
get_filename_component(TOP "${TOP}" REALPATH)

# Check for -DFAMILY=
if(FAMILY MATCHES "^esp32s[2-3]")
  include(${TOP}/hw/bsp/${FAMILY}/family.cmake)
  project(${PROJECT})
else()
  message(FATAL_ERROR "Invalid FAMILY specified: ${FAMILY}")
endif()


################################

idf_component_register(SRCS "main.c" "usb_descriptors.c"
                    INCLUDE_DIRS "."
                    REQUIRES freertos soc)

file(TO_NATIVE_PATH "${TOP}/hw/bsp/${FAMILY}/boards/${BOARD}/board.cmake" board_cmake)

if(EXISTS ${board_cmake})
    include(${board_cmake})
endif()

idf_component_get_property( FREERTOS_ORIG_INCLUDE_PATH freertos ORIG_INCLUDE_PATH)
target_include_directories(${COMPONENT_TARGET} PUBLIC
  "${FREERTOS_ORIG_INCLUDE_PATH}"
  "${TOP}/hw"
  "${TOP}/src"
)

target_sources(${COMPONENT_TARGET} PUBLIC
  "${TOP}/src/tusb.c"
  "${TOP}/src/common/tusb_fifo.c"
  "${TOP}/src/device/usbd.c"
  "${TOP}/src/device/usbd_control.c"
  "${TOP}/src/class/cdc/cdc_device.c"
  "${TOP}/src/class/dfu/dfu_rt_device.c"
  "${TOP}/src/class/hid/hid_device.c"
  "${TOP}/src/class/midi/midi_device.c"
  "${TOP}/src/class/msc/msc_device.c"
  "${TOP}/src/class/net/net_device.c"
  "${TOP}/src/class/usbtmc/usbtmc_device.c"
  "${TOP}/src/class/vendor/vendor_device.c"
  "${TOP}/src/portable/espressif/esp32sx/dcd_esp32sx.c"
)
