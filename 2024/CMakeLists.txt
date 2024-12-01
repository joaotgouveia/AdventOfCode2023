cmake_minimum_required(VERSION 3.23)

project(
    AdventOfCode2024
    VERSION 1.0
    LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

find_program(CLANG_TIDY NAMES "clang-tidy")
if(CLANG_TIDY)
    set(CLANG_TIDY_COMMAND "${CLANG_TIDY}" "-p" "${CMAKE_BINARY_DIR}")
endif()

find_program(CPPCHECK NAMES "cppcheck")
if(CPPCHECK)
    set(
        CPPCHECK_COMMAND
        "${CPPCHECK}"
        "--enable=warning,style,performance")
endif()

file(GLOB DAYS day*)
foreach(DAY IN ITEMS ${DAYS})
    add_subdirectory(${DAY})
endforeach()
