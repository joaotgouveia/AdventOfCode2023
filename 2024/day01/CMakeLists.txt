add_executable(day01p1 day01p1.cpp)
add_executable(day01p2 day01p2.cpp)
if(CLANG_TIDY_COMMAND)
    set_target_properties(day01p1 PROPERTIES CXX_CLANG_TIDY "${CLANG_TIDY_COMMAND}")
    set_target_properties(day01p2 PROPERTIES CXX_CLANG_TIDY "${CLANG_TIDY_COMMAND}")
endif()
if(CPPCHECK_COMMAND)
    set_target_properties(day01p1 PROPERTIES CXX_CPPCHECK "${CPPCHECK_COMMAND}")
    set_target_properties(day01p2 PROPERTIES CXX_CPPCHECK "${CPPCHECK_COMMAND}")
endif()
