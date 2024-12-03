cmake_minimum_required(VERSION 3.23)

project(
    AdventOfCode2024
    VERSION 1.0
    LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
set(DAY "" CACHE PATH "Only build the solutions for a specific day.")

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

if(DAY)
  add_subdirectory(${DAY})
else()
  add_subdirectory(day01)
  add_subdirectory(day02)
  add_subdirectory(day03)
endif()
