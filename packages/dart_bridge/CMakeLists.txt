cmake_minimum_required(VERSION 3.7 FATAL_ERROR)
project(waku_dart_bridge_library VERSION 1.0.0 LANGUAGES C)

set(HEADER_FILES
    "bridge.h"
    "include/uthash.h"
    "include/dart_api.h"
    "include/dart_native_api.h"
    "include/dart_tools_api.h"
    "include/dart_api_dl.h"
    "include/dart_version.h"
    "include/internal/dart_api_dl_impl.h"
)

add_library(waku_dart_bridge_library SHARED bridge.c bridge.def ${HEADER_FILES} "include/dart_api_dl.c")
target_include_directories(waku_dart_bridge_library PRIVATE "include" "include/internal")
target_link_libraries(waku_dart_bridge_library PUBLIC m)

set_target_properties(waku_dart_bridge_library PROPERTIES
    PUBLIC_HEADER bridge.h
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
    OUTPUT_NAME "waku_dart_bridge"
)
