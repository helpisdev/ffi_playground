cmake_minimum_required(VERSION 3.7 FATAL_ERROR)
project(waku_dart_bridge_library VERSION 1.0.0 LANGUAGES C)

set(INCLUDE_DIRS "../include" "../include/headers" "../include/headers/internal")

set(HEADER_FILES
    "../include/bridge.h"
    "../include/headers/uthash.h"
    "../include/headers/dart_api.h"
    "../include/headers/dart_native_api.h"
    "../include/headers/dart_tools_api.h"
    "../include/headers/dart_api_dl.h"
    "../include/headers/dart_version.h"
    "../include/headers/internal/dart_api_dl_impl.h"
)

set(SOURCES
    "src/lib.c"
    "../include/headers/dart_api_dl.c"
)

add_library(waku_dart_bridge_library SHARED "../include/bridge.def" ${HEADER_FILES} ${SOURCES})
target_include_directories(waku_dart_bridge_library PRIVATE ${INCLUDE_DIRS})
target_link_libraries(waku_dart_bridge_library PUBLIC m)

set_target_properties(waku_dart_bridge_library PROPERTIES
    PUBLIC_HEADER bridge.h
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
    OUTPUT_NAME "waku_dart_bridge"
)
